use std::io::{self, BufRead, Write};

fn parse_first_line(line: &str) -> i32 {
    let mut parts = line.split_whitespace();
    parts.next().unwrap().parse().unwrap()
}

fn parse_second_line(line: &str) -> Vec<i32> {
    let parts = line.split_whitespace();
    parts.map(|part| part.parse().unwrap()).collect()
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let _ = parse_first_line(&first_line);
    let pearls = parse_second_line(&second_line);

    let mut amazing_perals: Vec<i32> = pearls
        .iter()
        .scan(-1, |state, &x| {
            *state = std::cmp::max(*state, x);
            Some(*state)
        })
        .collect();
    amazing_perals.dedup();
    amazing_perals.reverse();
    let mut max_amazing = amazing_perals.len();

    for pearl in pearls.into_iter().rev() {
        while !amazing_perals.is_empty() && amazing_perals[amazing_perals.len() - 1] <= pearl {
            amazing_perals.pop();
        }
        amazing_perals.push(pearl);
        max_amazing = std::cmp::max(max_amazing, amazing_perals.len());
    }

    writeln!(stdout, "{}", max_amazing).expect("write stdout");
}
