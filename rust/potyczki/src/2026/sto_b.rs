use std::collections::{BinaryHeap, HashSet};
use std::io::{self, BufRead, Write};

fn parse_first_line(line: &str) -> (i32, i32, usize) {
    let mut parts = line.split_whitespace();
    let n = parts.next().unwrap().parse().unwrap();
    let m = parts.next().unwrap().parse().unwrap();
    let k = parts.next().unwrap().parse().unwrap();
    (n, m, k)
}

fn parse_line(line: &str) -> Vec<i64> {
    let parts = line.split_whitespace();
    parts.map(|part| part.parse().unwrap()).collect()
}

fn process_decreasing(pancakes: Vec<Vec<i64>>, k: usize) -> Vec<i64> {
    if pancakes.is_empty() {
        return vec![0; k + 1];
    }

    let mut heap = BinaryHeap::new();

    pancakes.iter().flatten().for_each(|x| heap.push(*x));
    std::iter::once(0)
        .chain((1..=k).scan(0, |state, _| {
            *state += heap.pop().unwrap_or(0);
            Some(*state)
        }))
        .collect()
}

fn process_increasing(pancakes: Vec<Vec<i64>>, k: usize) -> Vec<i64> {
    let mut dp = vec![0; k + 1];
    if pancakes.is_empty() {
        return dp;
    }
    let m = pancakes[0].len();
    let total_pancakes = pancakes.iter().map(|c| c.len()).sum();

    let mut column_heap = vec![BinaryHeap::new(); m];

    pancakes.iter().enumerate().for_each(|(row, stack)| {
        stack
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .take(k)
            .enumerate()
            .for_each(|(i, x)| column_heap[i].push((x, row)))
    });

    let mut visited_rows: HashSet<usize> = HashSet::new();

    let mut full_row_sum = 0;
    for pancakes_to_take in 1..=k {
        if pancakes_to_take > total_pancakes {
            dp[pancakes_to_take] = dp[pancakes_to_take - 1];
            continue;
        }

        let column = (pancakes_to_take - 1) % m;

        while visited_rows.contains(&column_heap[column].peek().unwrap().1) {
            column_heap[column].pop();
        }

        let (best_size, best_row) = column_heap[column].peek().unwrap();
        dp[pancakes_to_take] = full_row_sum + best_size;

        if column == m - 1 {
            full_row_sum += best_size;
            visited_rows.insert(*best_row);
        }
    }

    dp
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();

    let (n, _m, k) = parse_first_line(&first_line);

    let mut pancakes: Vec<Vec<i64>> = Vec::new();

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        pancakes.push(parse_line(&line));
    }

    let (decreasing, increasing) = pancakes.into_iter().partition(|stack| {
        stack.len() == 1 || stack[0] >= *stack.iter().find(|&&x| x != stack[0]).unwrap_or(&stack[0])
    });

    let decreasing_dp = process_decreasing(decreasing, k);
    let increasing_dp = process_increasing(increasing, k);

    let result = std::iter::zip(decreasing_dp, increasing_dp.iter().rev())
        .map(|(a, b)| a + b)
        .max()
        .unwrap();

    writeln!(stdout, "{}", result).expect("write stdout");
}
