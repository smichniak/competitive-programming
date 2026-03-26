use std::collections::HashMap;
use std::hash::Hash;

/// Disjoint-set (union–find) with path compression and union by component size.
///
/// Elements are identified by `T` (must be `Hash` + `Eq` for the internal map).
/// `find`, `union`, and `component_size` take `&mut self` so path compression can run.
pub struct UnionFind<T> {
    elem_to_idx: HashMap<T, usize>,
    idx_to_elem: Vec<T>,
    parent: Vec<usize>,
    /// Valid only at a root index: size of that component.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes_and_union_by_size() {
        let mut uf = UnionFind::new();
        for x in ['a', 'b', 'c', 'd'] {
            uf.make_set(x);
        }
        assert_eq!(uf.component_size(&'a'), Some(1));
        uf.union(&'a', &'b');
        assert_eq!(uf.component_size(&'a'), Some(2));
        assert_eq!(uf.component_size(&'b'), Some(2));
        uf.union(&'c', &'d');
        uf.union(&'a', &'c');
        assert_eq!(uf.component_size(&'d'), Some(4));
        assert!(uf.same_set(&'a', &'d'));
        let r = *uf.find(&'b').unwrap();
        assert_eq!(uf.find(&'c'), Some(&r));
        assert_eq!(uf.component_size(&r), Some(4));
    }

    #[test]
    fn same_set_and_idempotent_union() {
        let mut uf = UnionFind::new();
        uf.make_set(1);
        uf.make_set(2);
        assert!(uf.union(&1, &2));
        assert!(!uf.union(&1, &2));
        assert!(uf.same_set(&1, &2));
    }
}
