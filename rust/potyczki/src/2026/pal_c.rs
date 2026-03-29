use std::io::{self, BufRead, Write};

const NIE: &str = "NIE";
const MY_STR: &str = "AAPAPP";

fn parse_first_line(line: &str) -> i32 {
    let mut parts = line.split_whitespace();
    parts.next().unwrap().parse().unwrap()
}
fn parse_line(line: &str) -> (usize, usize) {
    let mut parts = line.split_whitespace();
    let n = parts.next().unwrap().parse().unwrap();
    let k = parts.next().unwrap().parse().unwrap();

    (n, k)
}

fn longer_string(n: usize, k: usize) -> String {
    if k < 4 {
        return NIE.to_string();
    }

    let as_at_start = k - 2;
    let repeats = (n - as_at_start) / MY_STR.len();
    let left = n - as_at_start - MY_STR.len() * repeats;

    ("A".repeat(as_at_start) + &MY_STR.repeat(repeats) + &MY_STR[0..left]).to_string()
}

fn process_test(n: usize, k: usize) -> String {
    let l1 = ["P"];
    let l2 = ["PA", "PP"];
    let l3 = ["", "PPA", "PPP"];
    let l4 = ["", "PPAA", "PPPA", "PPPP"];
    let l5 = ["", "", "PPPAP", "PPPPA", "PPPPP"];
    let l6 = ["", "", "PPPAPA", "PPPPAP", "PPPPPA", "PPPPPP"];
    let l7 = [
        "", "", "PPPAPAA", "PPPPAPA", "PPPPPAP", "PPPPPPA", "PPPPPPP",
    ];
    let l8 = [
        "", "", "PPPAPAAA", "PPPPAPAA", "PPPPPAPP", "PPPPPPAP", "PPPPPPPA", "PPPPPPPP",
    ];

    let res = match n {
        0 => unreachable!(),
        1 => l1[k - 1].to_string(),
        2 => l2[k - 1].to_string(),
        3 => l3[k - 1].to_string(),
        4 => l4[k - 1].to_string(),
        5 => l5[k - 1].to_string(),
        6 => l6[k - 1].to_string(),
        7 => l7[k - 1].to_string(),
        8 => l8[k - 1].to_string(),
        _ => longer_string(n, k),
    };

    if res.is_empty() { NIE.to_string() } else { res }
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();

    let t = parse_first_line(&first_line);
    for _ in 0..t {
        let next_line = lines.next().unwrap().unwrap();
        let (n, k) = parse_line(&next_line);
        writeln!(stdout, "{}", process_test(n, k)).expect("write stdout");
    }
}
