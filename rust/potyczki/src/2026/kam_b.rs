use std::{
    collections::HashSet,
    io::{self, BufRead, Write},
    vec,
};

use std::collections::HashMap;
use std::hash::Hash;

pub struct UnionFind<T> {
    elem_to_idx: HashMap<T, usize>,
    idx_to_elem: Vec<T>,
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl<T: Clone + Eq + Hash> UnionFind<T> {
    pub fn new() -> Self {
        Self {
            elem_to_idx: HashMap::new(),
            idx_to_elem: Vec::new(),
            parent: Vec::new(),
            size: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.idx_to_elem.len()
    }

    pub fn is_empty(&self) -> bool {
        self.idx_to_elem.is_empty()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.elem_to_idx.contains_key(item)
    }

    /// Inserts `item` as its own singleton set if it is not already present.
    pub fn make_set(&mut self, item: T) {
        if self.elem_to_idx.contains_key(&item) {
            return;
        }
        let idx = self.idx_to_elem.len();
        self.elem_to_idx.insert(item.clone(), idx);
        self.idx_to_elem.push(item);
        self.parent.push(idx);
        self.size.push(1);
    }

    /// Root representative for `item`’s set, or `None` if `item` is unknown.
    pub fn find(&mut self, item: &T) -> Option<&T> {
        let idx = *self.elem_to_idx.get(item)?;
        let root = self.find_root(idx);
        Some(&self.idx_to_elem[root])
    }

    /// Number of elements in the component containing `item`, or `None` if unknown.
    pub fn component_size(&mut self, item: &T) -> Option<usize> {
        let idx = *self.elem_to_idx.get(item)?;
        let root = self.find_root(idx);
        Some(self.size[root])
    }

    /// Merges the sets of `a` and `b`. Returns `true` if they were separate and are now merged.
    pub fn union(&mut self, a: &T, b: &T) -> bool {
        let Some(&ia) = self.elem_to_idx.get(a) else {
            return false;
        };
        let Some(&ib) = self.elem_to_idx.get(b) else {
            return false;
        };
        let ra = self.find_root(ia);
        let rb = self.find_root(ib);
        if ra == rb {
            return false;
        }
        let (small, large) = if self.size[ra] < self.size[rb] {
            (ra, rb)
        } else {
            (rb, ra)
        };
        self.parent[small] = large;
        self.size[large] += self.size[small];
        true
    }

    pub fn same_set(&mut self, a: &T, b: &T) -> bool {
        match (self.elem_to_idx.get(a), self.elem_to_idx.get(b)) {
            (Some(&ia), Some(&ib)) => self.find_root(ia) == self.find_root(ib),
            _ => false,
        }
    }

    fn find_root(&mut self, mut idx: usize) -> usize {
        let mut root = idx;
        while self.parent[root] != root {
            root = self.parent[root];
        }
        while self.parent[idx] != root {
            let p = self.parent[idx];
            self.parent[idx] = root;
            idx = p;
        }
        root
    }
}

impl<T: Clone + Eq + Hash> Default for UnionFind<T> {
    fn default() -> Self {
        Self::new()
    }
}

const TAK: &str = "TAK";
const NIE: &str = "NIE";

fn parse_first_line(line: &str) -> i32 {
    let mut parts = line.split_whitespace();
    parts.next().unwrap().parse().unwrap()
}

fn parse_district_start(line: &str) -> (usize, i32, usize) {
    let mut parts = line.split_whitespace();
    let n = parts.next().unwrap().parse().unwrap();
    let m = parts.next().unwrap().parse().unwrap();
    let k = parts.next().unwrap().parse().unwrap();
    (n, m, k)
}

fn parse_winners(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .map(|part| part.parse::<usize>().unwrap() - 1)
        .collect()
}

fn parse_graph_pairs(line: &str) -> (usize, usize) {
    let mut parts = line.split_whitespace();
    let u = parts.next().unwrap().parse().unwrap();
    let v = parts.next().unwrap().parse().unwrap();
    (u, v)
}

fn process_district(lines: &mut std::io::Lines<std::io::StdinLock<'_>>) -> &'static str {
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let (n, m, k) = parse_district_start(&first_line);
    let winners = parse_winners(&second_line);
    let mut pairs = Vec::new();
    for _ in 0..m {
        pairs.push(parse_graph_pairs(&lines.next().unwrap().unwrap()));
    }

    let mut graph = vec![HashSet::new(); n];
    for (u, v) in pairs {
        graph[u - 1].insert(v - 1);
        graph[v - 1].insert(u - 1);
    }

    let mut party_chunks = UnionFind::new();
    let mut party_chunk_count = vec![0; k];

    let mut party_wins = vec![Vec::new(); k];

    for city in 0..n {
        party_chunks.make_set(city);
        party_chunk_count[winners[city]] += 1;
    }
    let mut one_chunkers = Vec::new();
    let mut removed_parties = HashSet::new();

    for city in 0..n {
        for neighbor in &graph[city] {
            if winners[city] == winners[*neighbor] && !party_chunks.same_set(&city, neighbor) {
                party_chunks.union(&city, neighbor);
                party_chunk_count[winners[city]] -= 1;
            }
        }
        party_wins[winners[city]].push(city);
    }

    for (party, chunk_count) in party_chunk_count.iter().enumerate() {
        if *chunk_count == 1 {
            one_chunkers.push(party);
        } else if *chunk_count == 0 {
            removed_parties.insert(party);
        }
    }

    let mut neighbor_to_last;
    let mut collected_reps: HashMap<usize, usize> = HashMap::new();
    let mut removed_in_this_chunk = HashSet::new();

    while let Some(party) = one_chunkers.pop() {
        if !removed_parties.contains(&party) {
            neighbor_to_last = party_wins[party]
                .iter()
                .flat_map(|city| graph[*city].iter())
                .any(|neighbor| removed_in_this_chunk.contains(neighbor));

            if !neighbor_to_last {
                collected_reps = HashMap::new();
            }

            for city in &party_wins[party] {
                removed_in_this_chunk.insert(city);
            }

            for city in &party_wins[party] {
                for &neighbor in &graph[*city] {
                    let neighbor_party = winners[neighbor];
                    if neighbor_party != party && !removed_parties.contains(&neighbor_party) {
                        if let Some(part_rep) = collected_reps.get(&neighbor_party) {
                            if !party_chunks.same_set(part_rep, &neighbor) {
                                party_chunks.union(part_rep, &neighbor);
                                party_chunk_count[neighbor_party] -= 1;
                                if party_chunk_count[neighbor_party] == 1 {
                                    one_chunkers.push(neighbor_party);
                                }
                            }
                        } else {
                            collected_reps.insert(neighbor_party, neighbor);
                            if party_chunk_count[neighbor_party] == 1 {
                                one_chunkers.push(neighbor_party);
                            }
                        }
                    }
                }
            }

            removed_parties.insert(party);
        }
    }

    if removed_parties.len() != k {
        return NIE;
    }

    TAK
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();

    let t = parse_first_line(&first_line);
    for _ in 0..t {
        writeln!(stdout, "{}", process_district(&mut lines)).expect("write stdout");
    }
}
