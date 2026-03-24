use std::collections::BinaryHeap;
use std::io::{self, BufRead, Write};

fn parse_first_line(line: &str) -> (i32, i32) {
    let mut parts = line.split_whitespace();
    let n = parts.next().unwrap().parse().unwrap();
    let k = parts.next().unwrap().parse().unwrap();
    (n, k)
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

    let (_n, k) = parse_first_line(&first_line);
    let mut nums = parse_second_line(&second_line);

    let mut heap = BinaryHeap::new();
    for (i, x) in nums.iter().enumerate() {
        heap.push((*x, i));
    }

    let mut result = 0;

    while !heap.is_empty() {
        let (_height, index) = heap.pop().unwrap();

        if index > 0 && nums[index - 1] + k < nums[index] {
            let to_add = nums[index] - k - nums[index - 1];
            nums[index - 1] += to_add;
            result += to_add;
            heap.push((nums[index - 1], index - 1));
        }

        if index < nums.len() - 1 && nums[index + 1] + k < nums[index] {
            let to_add = nums[index] - k - nums[index + 1];
            nums[index + 1] += to_add;
            result += to_add;
            heap.push((nums[index + 1], index + 1));
        }
    }

    writeln!(stdout, "{}", result).expect("write stdout");
}
