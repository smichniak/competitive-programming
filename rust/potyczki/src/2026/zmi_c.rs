use std::io::{self, BufRead, Write};

fn parse_line(line: &str) -> (i32, i32, i32, i32) {
    let mut parts = line.split_whitespace();
    let x1 = parts.next().unwrap().parse().unwrap();
    let y1 = parts.next().unwrap().parse().unwrap();
    let x2 = parts.next().unwrap().parse().unwrap();
    let y2 = parts.next().unwrap().parse().unwrap();
    (x1, y1, x2, y2)
}

fn main() {
    let stdin = io::stdin().lock();
    let mut stdout = io::stdout().lock();

    let line = stdin.lines().next().unwrap().unwrap();
    let (round, day, hour, minute) = parse_line(&line);

    const MINUTES_IN_HOUR: i32 = 60;
    const HOURS_IN_DAY: i32 = 24;
    const MINUTES_IN_DAY: i32 = MINUTES_IN_HOUR * HOURS_IN_DAY;

    const ROUND_ONE_START: i32 = 0;
    const ROUND_ONE_END: i32 = MINUTES_IN_DAY / 2 + MINUTES_IN_DAY - 1;

    const ROUND_TWO_START: i32 = ROUND_ONE_START + MINUTES_IN_DAY;
    const ROUND_TWO_END: i32 = ROUND_ONE_END + MINUTES_IN_DAY;

    const ROUND_THREE_START: i32 = ROUND_TWO_START + MINUTES_IN_DAY;
    const ROUND_THREE_END: i32 = ROUND_TWO_END + MINUTES_IN_DAY;

    const ROUND_FOUR_START: i32 = ROUND_THREE_START + MINUTES_IN_DAY;
    const ROUND_FOUR_END: i32 = ROUND_THREE_END + MINUTES_IN_DAY;

    const ROUND_FIVE_START: i32 = ROUND_FOUR_START + MINUTES_IN_DAY;
    const ROUND_FIVE_END: i32 = ROUND_FOUR_END + 2 * MINUTES_IN_DAY;

    const TIME_CHANGE: i32 = ROUND_FIVE_START + 2 * MINUTES_IN_DAY - 10 * MINUTES_IN_HOUR;

    let start_time =
        (day - 23) * MINUTES_IN_DAY + hour * MINUTES_IN_HOUR + minute - MINUTES_IN_DAY / 2;

    let mut time_left = match round {
        1 => ROUND_ONE_END,
        2 => ROUND_TWO_END,
        3 => ROUND_THREE_END,
        4 => ROUND_FOUR_END,
        5 => ROUND_FIVE_END,
        _ => unreachable!(),
    } - start_time
        + 1;

    if round == 5 && start_time < TIME_CHANGE {
        time_left -= MINUTES_IN_HOUR;
    }

    writeln!(stdout, "{}", time_left).expect("write stdout");
}
