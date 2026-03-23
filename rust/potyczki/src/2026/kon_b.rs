use std::{
    io::{self, BufRead, Write},
    iter::zip,
};

fn parse_first_line(line: &str) -> (i32, i32) {
    let mut parts = line.split_whitespace();
    let k = parts.next().unwrap().parse().unwrap();
    let n1 = parts.next().unwrap().parse().unwrap();
    (k, n1)
}

fn parse_line(line: &str) -> Vec<i32> {
    let mut parts = line.split_whitespace();
    parts.next();
    parts.map(|part| part.parse().unwrap()).collect()
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let mut lines = stdin.lines();
    let first_line = lines.next().unwrap().unwrap();
    let (k, n1) = parse_first_line(&first_line);

    if k == 1 {
        writeln!(stdout, "{}", n1).expect("write stdout");
        return;
    }

    let mut days: Vec<Vec<i32>> = Vec::new();
    days.push(vec![0; n1 as usize]);

    for _ in 0..(k - 1) {
        let line = lines.next().unwrap().unwrap();
        days.push(parse_line(&line));
    }

    let mut current_day_index = k - 1;
    let mut next_day_people: Vec<i32> = vec![];

    let mut index: i32 = 0;
    let mut free_people: i32 = 0;

    while current_day_index >= 0 {
        let mut current_day_people = vec![0; days[current_day_index as usize].len()];

        if !next_day_people.is_empty() {
            for (people, continuation) in
                zip(next_day_people, &days[current_day_index as usize + 1])
            {
                if *continuation > 0 {
                    current_day_people[*continuation as usize - 1] += people;
                } else {
                    free_people += people;
                }
            }
        }

        for people in &mut current_day_people {
            if *people == 0 {
                *people += 1;
                if free_people == 0 {
                    index += 1;
                } else {
                    free_people -= 1;
                }
            }
        }

        current_day_index -= 1;
        next_day_people = current_day_people;
    }

    writeln!(stdout, "{}", index).expect("write stdout");
}
