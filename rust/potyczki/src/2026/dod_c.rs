use std::io::{self, BufRead, Write};

fn parse_lines(line: &str) -> Vec<i64> {
    line.split("")
        .filter(|c| !c.is_empty())
        .map(|part| part.parse().unwrap())
        .collect()
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let third_line = lines.next().unwrap().unwrap();

    let first_num = parse_lines(&first_line);
    let second_num = parse_lines(&second_line);
    let third_num = parse_lines(&third_line);

    let n = first_num.len();

    let mut dp: Vec<Vec<i64>> = vec![vec![0; n]; 2];
    // dp[d][i] - how many good additions end at index `i`, if we add additional `d`
    // result = sum_{i=0 to n} dp[0][i]

    for (i, (a, (b, c))) in first_num
        .into_iter()
        .zip(second_num.into_iter().zip(third_num))
        .enumerate()
    {
        for d in 0..=1 {
            let mut sum = a + b + d;
            let mut carry = 0;
            if sum >= 10 {
                sum -= 10;
                carry += 1;
            }

            if sum == c {
                if carry == 0 {
                    dp[d as usize][i] += 1;
                }

                if i > 0 {
                    dp[d as usize][i] += dp[carry][i - 1]
                }
            }
        }
    }

    let result: i64 = dp[0].iter().sum();
    writeln!(stdout, "{}", result).expect("write stdout");
}
