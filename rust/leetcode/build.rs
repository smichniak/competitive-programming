//! Emit `_mod_autogen.rs` in `src/problems/` so `pub mod pNNNN_*` resolves next to each source file.

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn is_problem_module_stem(stem: &str) -> bool {
    let b = stem.as_bytes();
    if b.len() < 6 || b[0] != b'p' {
        return false;
    }
    b[1..5].iter().all(|&x| x.is_ascii_digit()) && b[5] == b'_'
}

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let problems_dir = Path::new(&manifest_dir).join("src/problems");
    let out_path = problems_dir.join("_mod_autogen.rs");

    let mut stems: Vec<String> = fs::read_dir(&problems_dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", problems_dir.display()))
        .filter_map(|e| e.ok())
        .filter_map(|e| {
            let path = e.path();
            if path.file_name().and_then(|s| s.to_str()) == Some("mod.rs") {
                return None;
            }
            if path.extension().and_then(|s| s.to_str()) != Some("rs") {
                return None;
            }
            let stem = path.file_stem()?.to_str()?.to_string();
            if !is_problem_module_stem(&stem) {
                return None;
            }
            Some(stem)
        })
        .collect();

    stems.sort();
    stems.dedup();

    let mut f = fs::File::create(&out_path).expect("create _mod_autogen.rs");
    for stem in stems {
        writeln!(f, "pub mod {stem};").unwrap();
    }

    println!("cargo:rerun-if-changed={}", problems_dir.display());
}
