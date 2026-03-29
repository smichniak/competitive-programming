use std::io::{self, BufRead, Write};

fn get_divisors(n: usize) -> Vec<usize> {
    if n <= 1 {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut i = 1usize;
    while i * i <= n {
        if n % i == 0 {
            out.push(i);
            let j = n / i;
            if j != i {
                out.push(j);
            }
        }
        i += 1;
    }
    out.sort_unstable();
    out.pop();
    out
}

fn parse_first_line(line: &str) -> usize {
    let mut parts = line.split_whitespace();
    parts.next().unwrap().parse().unwrap()
}

fn parse_second_line(line: &str) -> Vec<usize> {
    let parts = line.split_whitespace();
    parts.map(|part| part.parse().unwrap()).collect()
}

fn can_waves_be_this_wide(ambers: &[usize], wave_width: usize) -> bool {
    let mut active_waves = 0;
    let mut waves_added_at = vec![0; ambers.len()];

    for (i, &amber) in ambers.iter().enumerate() {
        if active_waves > amber {
            return false;
        } else {
            waves_added_at[i] = amber - active_waves;
            active_waves += amber - active_waves;
        }

        if i + 1 >= wave_width {
            active_waves -= waves_added_at[i + 1 - wave_width];
        }
    }

    active_waves == 0
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();

    let _n = parse_first_line(&first_line);
    let ambers = parse_second_line(&second_line);
    let total_ambers: usize = ambers.iter().sum();
    let max_amber = *ambers.iter().max().unwrap();
    let divisors = get_divisors(total_ambers)
        .into_iter()
        .filter(|&x| x <= total_ambers.div_ceil(max_amber))
        .chain(std::iter::once(total_ambers));

    let result = divisors
        .rev()
        .find(|&width| can_waves_be_this_wide(&ambers, width))
        .unwrap_or(1);

    writeln!(stdout, "{}", result).expect("write stdout");
}

// wave_widh * num_wave = total_ambers
// num_wave >= max_amber
// 1 / num_wave <= 1 / max_amber
// wave_width = total_ambers / num_wave <= total_ambers / max_amber
