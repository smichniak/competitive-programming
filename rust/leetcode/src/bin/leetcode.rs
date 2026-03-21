//! CLI: `list` / `new` / `run` — mirror of `python -m leetcode`.
//!
//! Run from workspace root `rust/`: `cargo run -p leetcode -- <command>`

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{self, Command, Stdio};

fn main() {
    let code = match run() {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    };
    process::exit(code);
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        return Err("usage: leetcode <list|new|run> ... (or: cargo run -p leetcode -- ...)".into());
    }
    let cmd = args.remove(0);
    match cmd.as_str() {
        "list" => cmd_list(),
        "new" => cmd_new(args),
        "run" => cmd_run(args),
        _ => Err(format!("unknown command {cmd:?}")),
    }
}

fn workspace_root() -> Result<PathBuf, String> {
    let mut dir = env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let leetcode_manifest = dir.join("leetcode/Cargo.toml");
        let ws = dir.join("Cargo.toml");
        if leetcode_manifest.is_file() && ws.is_file() {
            let txt = fs::read_to_string(&ws).map_err(|e| e.to_string())?;
            if txt.contains("[workspace]") {
                return Ok(dir);
            }
        }
        if !dir.pop() {
            break;
        }
    }
    Err(
        "could not find rust workspace root (expected .../rust with leetcode/Cargo.toml); cd rust/ and retry"
            .into(),
    )
}

fn problems_dir() -> Result<PathBuf, String> {
    Ok(workspace_root()?.join("leetcode/src/problems"))
}

fn normalize_snake(raw: &str) -> Result<String, String> {
    let mut s = raw.trim().to_lowercase().replace('-', "_");
    s.retain(|c| c.is_ascii_alphanumeric() || c == '_');
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s = s.trim_matches('_').to_string();
    if s.is_empty() {
        Err("slug is empty after normalization".into())
    } else {
        Ok(s)
    }
}

fn snake_to_hyphen(snake: &str) -> String {
    snake.replace('_', "-")
}

fn slug_to_title(snake: &str) -> String {
    snake
        .split('_')
        .filter(|p| !p.is_empty())
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn scaffold_path(number: u32, slug_raw: &str) -> Result<PathBuf, String> {
    let snake = normalize_snake(slug_raw)?;
    Ok(problems_dir()?.join(format!("p{number:04}_{snake}.rs")))
}

fn render_new_file(number: u32, slug_raw: &str) -> Result<String, String> {
    let snake = normalize_snake(slug_raw)?;
    let hyphen = snake_to_hyphen(&snake);
    let title = slug_to_title(&snake);
    Ok(format!(
        r#"//! LeetCode {number}. {title}.
//!
//! https://leetcode.com/problems/{hyphen}/

pub struct Solution;

// impl Solution {{ ... }}

#[cfg(test)]
mod tests {{
    use super::Solution;

    #[test]
    fn stub() {{
        let _ = Solution;
        // Replace with real tests once you implement the solution.
    }}
}}
"#
    ))
}

fn is_problem_module_stem(stem: &str) -> bool {
    let b = stem.as_bytes();
    if b.len() < 6 || b[0] != b'p' {
        return false;
    }
    b[1..5].iter().all(|&x| x.is_ascii_digit()) && b[5] == b'_'
}

fn is_problem_filename(name: &str) -> bool {
    name.ends_with(".rs")
        && name != "mod.rs"
        && !name.starts_with('_')
        && name.strip_suffix(".rs").is_some_and(is_problem_module_stem)
}

fn iter_problem_paths() -> Result<Vec<PathBuf>, String> {
    let dir = problems_dir()?;
    let mut paths: Vec<PathBuf> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(is_problem_filename)
        })
        .collect();
    paths.sort();
    Ok(paths)
}

/// `p0025_foo_bar.rs` → (25, "foo_bar")
fn parse_problem_stem(name: &str) -> Option<(u32, String)> {
    let name = name.strip_suffix(".rs")?;
    let rest = name.strip_prefix('p')?;
    let num_str = rest.get(..4)?;
    if !num_str.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let n: u32 = num_str.parse().ok()?;
    let slug = rest.get(5..)?; // after `NNNN_`
    if rest.as_bytes().get(4).is_none_or(|&b| b != b'_') {
        return None;
    }
    Some((n, slug.to_string()))
}

fn normalize_slug_query(s: &str) -> String {
    s.trim().to_lowercase().replace('-', "_")
}

fn resolve_problem_file(query: &str) -> Result<PathBuf, String> {
    let q = query.trim();
    if q.is_empty() {
        return Err("empty problem query".into());
    }

    if q.chars().all(|c| c.is_ascii_digit()) {
        let n: u32 = q.parse().map_err(|_| "invalid problem number")?;
        let padded = format!("p{n:04}_");
        let matches: Vec<PathBuf> = iter_problem_paths()?
            .into_iter()
            .filter(|p| {
                p.file_name()
                    .and_then(|n| n.to_str())
                    .is_some_and(|name| name.starts_with(&padded))
            })
            .collect();
        return match matches.len() {
            0 => Err(format!(
                "no solution file for problem {n} (expected {padded}*.rs under {})",
                problems_dir()?.display()
            )),
            1 => Ok(matches[0].clone()),
            _ => Err(format!("multiple files for problem {n}: {matches:?}")),
        };
    }

    let needle = normalize_slug_query(q);
    let mut candidates: Vec<PathBuf> = Vec::new();
    for p in iter_problem_paths()? {
        let Some(fname) = p.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        let Some((_n, slug)) = parse_problem_stem(fname) else {
            continue;
        };
        let slug_lower = slug.to_lowercase();
        if needle == slug_lower || slug_lower.contains(&needle) {
            candidates.push(p);
        }
    }

    match candidates.len() {
        0 => Err(format!(
            "no solution file matching slug {q:?} (try `leetcode list`)"
        )),
        1 => Ok(candidates.remove(0)),
        _ => Err(format!("ambiguous slug {q:?}: {candidates:?}")),
    }
}

fn cmd_list() -> Result<(), String> {
    for p in iter_problem_paths()? {
        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        println!("{stem}");
    }
    Ok(())
}

fn cmd_new(mut args: Vec<String>) -> Result<(), String> {
    let mut force = false;
    args.retain(|a| {
        if a == "--force" {
            force = true;
            return false;
        }
        true
    });
    let number: u32 = args
        .first()
        .ok_or("usage: leetcode new <number> <slug> [--force]")?
        .parse()
        .map_err(|_| "problem number must be a positive integer")?;
    if number < 1 {
        return Err("problem number must be positive".into());
    }
    let slug = args
        .get(1)
        .ok_or("usage: leetcode new <number> <slug> [--force]")?
        .clone();

    let path = scaffold_path(number, &slug)?;
    if path.exists() && !force {
        return Err(format!("file already exists: {}", path.display()));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let text = render_new_file(number, &slug)?;
    fs::write(&path, text).map_err(|e| e.to_string())?;
    println!("{}", path.display());
    Ok(())
}

fn cmd_run(mut args: Vec<String>) -> Result<(), String> {
    let problem = args
        .first()
        .ok_or("usage: leetcode run <number|slug> [-- <args for test runner after -->]")?
        .clone();
    args.remove(0);
    let mut extra: Vec<String> = Vec::new();
    if let Some(pos) = args.iter().position(|a| a == "--") {
        extra = args.drain(pos + 1..).collect();
        args.pop();
    }

    let path = resolve_problem_file(&problem)?;
    let fname = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or("invalid path")?;
    let Some((n, _slug)) = parse_problem_stem(fname) else {
        return Err(format!("not a problem file: {}", path.display()));
    };
    let filter = format!("p{n:04}");

    let root = workspace_root()?;
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&root);
    cmd.arg("test");
    cmd.arg("-p");
    cmd.arg("leetcode");
    cmd.arg(&filter);
    if !extra.is_empty() {
        cmd.arg("--");
        cmd.args(extra);
    }
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd
        .status()
        .map_err(|e| format!("failed to spawn `cargo test` (is Rust installed?): {e}"))?;
    if !status.success() {
        return Err(format!("cargo test exited with {status}"));
    }
    Ok(())
}
