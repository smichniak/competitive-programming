# Competitive programming

Personal archive of **LeetCode-style solutions** and small **reusable building blocks**. The same workflow exists in **Rust** and **Python**: one file per problem, in-file tests, and a tiny CLI to list problems, scaffold new files, and run only the tests you care about.

## Layout

| Path | Role |
|------|------|
| `rust/` | Cargo workspace; LeetCode crate under `leetcode/` |
| `rust/leetcode/src/problems/` | One module per problem: `pNNNN_snake_case.rs` |
| `python/leetcode/problems/` | One module per problem: `NNNN_snake_case.py` |
| `rust/data_structures/`, `python/data_structures/` | Shared primitives (grow over time) |

Rust uses a `p` prefix on filenames so module names are valid (`p0025_…` → `mod p0025_…`). Python uses a four-digit numeric prefix only. Adding a new `pNNNN_*.rs` file is picked up on the next build via `build.rs` (generated `mod` list).

## Prerequisites

- **Rust**: stable toolchain with `cargo` ([rustup](https://rustup.rs/))
- **Python**: 3.14+ (see `python/pyproject.toml`)

## Rust

Working directory: **`rust/`** (workspace root).

```bash
# All problem tests
cargo test -p leetcode

# Filter by problem id (matches test names like p0025_*)
cargo test -p leetcode p0025
```

Helper binary (same idea as `python -m leetcode`):

```bash
cargo run -p leetcode -- list
cargo run -p leetcode -- new 1234 my-problem-slug
cargo run -p leetcode -- run 25              # by number
cargo run -p leetcode -- run reverse_nodes    # by slug fragment
# Extra args after -- go to the test runner
cargo run -p leetcode -- run 25 -- --nocapture
```

Release builds are optional; for local iteration, debug `cargo test` is enough.

## Python

```bash
cd python
python -m venv .venv && source .venv/bin/activate   # optional
pip install -e ".[dev]"   # installs pytest
```

```bash
python -m leetcode list
python -m leetcode new 1234 my-problem-slug
python -m leetcode run 25
python -m leetcode run reverse_nodes -- -k test_name
```

Or run pytest directly on a file under `leetcode/problems/` if you prefer.

## License

MIT — see [LICENSE](LICENSE).
