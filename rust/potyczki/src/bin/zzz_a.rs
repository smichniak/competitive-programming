//! Skeleton binary; rename to `{code}_{a|b|c}.rs` (e.g. `abc_b.rs` → `cargo run --bin abc_b`).

use std::io::{self, Write};

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for line in stdin.lines().map_while(Result::ok) {
        writeln!(stdout, "{line}").expect("write stdout");
    }
}
