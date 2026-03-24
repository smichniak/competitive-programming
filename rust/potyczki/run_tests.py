#!/usr/bin/env python3
"""
Run I/O tests under test/<binary_name>/.

For every *.in file (recursively), runs the matching Cargo binary with that
input and compares stdout to <same_stem>.out. Exit code 1 if any test fails.

Usage:
  ./run_tests.py              # all problems that have a folder under test/
  ./run_tests.py kon_b zmi_c  # only these binaries (if their test dirs exist)
  ./run_tests.py --test-dir test/sto_b/tests sto_b  # scan DIR recursively for *.in
  ./run_tests.py -q           # quiet: only failures + totals

Default mode: each directory test/<name>/ matches a Cargo [[bin]] name; all *.in files
under that tree are run; each output is compared to the sibling <stem>.out next to the
.in file.

With --test-dir: DIR is the exact folder that contains tests or any ancestor; all *.in
files under DIR are found recursively. Pass exactly one binary name (positional) as
the Cargo --bin to run. Pairs use the same stem with .out in the same directory as the
.in (no extra path suffixes).

Respects CARGO_TARGET_DIR if set (same as cargo).
"""

from __future__ import annotations

import argparse
import os
import subprocess
import sys
import time
from pathlib import Path


def repo_layout(script_path: Path) -> tuple[Path, Path, Path]:
    potyczki_root = script_path.resolve().parent
    workspace_root = potyczki_root.parent
    test_root = potyczki_root / "test"
    return potyczki_root, workspace_root, test_root


def discover_problems(test_root: Path, only: list[str] | None) -> list[str]:
    if not test_root.is_dir():
        return []
    names = sorted(p.name for p in test_root.iterdir() if p.is_dir())
    if only:
        wanted = set(only)
        names = [n for n in names if n in wanted]
        missing = wanted - set(names)
        for m in sorted(missing):
            print(f"warning: no test directory {test_root / m}/", file=sys.stderr)
    return names


def normalize_output(data: bytes) -> bytes:
    """Strip a single trailing newline, matching common judge behavior."""
    if data.endswith(b"\r\n"):
        return data[:-2]
    if data.endswith(b"\n"):
        return data[:-1]
    if data.endswith(b"\r"):
        return data[:-1]
    return data


def run_one(
    binary: Path,
    in_path: Path,
    out_path: Path,
    timeout: float | None,
) -> tuple[bool, str, float]:
    expected = out_path.read_bytes()
    t0 = time.perf_counter()
    data_in = in_path.read_bytes()
    try:
        proc = subprocess.run(
            [str(binary)],
            input=data_in,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            timeout=timeout,
            cwd=binary.parent,
        )
    except subprocess.TimeoutExpired:
        elapsed = time.perf_counter() - t0
        return False, f"timeout after {timeout}s", elapsed

    elapsed = time.perf_counter() - t0
    if proc.returncode != 0:
        err = proc.stderr.decode(errors="replace").strip()
        msg = f"exit {proc.returncode}"
        if err:
            msg += f", stderr: {err[:500]}"
        return False, msg, elapsed

    got = proc.stdout
    exp = expected
    if normalize_output(got) != normalize_output(exp):
        g = got.decode(errors="replace")
        e = exp.decode(errors="replace")
        return (
            False,
            "wrong answer\n"
            f"  expected ({len(exp)} bytes): {e!r}\n"
            f"  got      ({len(got)} bytes): {g!r}",
            elapsed,
        )
    return True, "", elapsed


def build_binaries(workspace_root: Path, names: list[str], release: bool) -> bool:
    if not names:
        return True
    cmd = ["cargo", "build", "-p", "potyczki"]
    if release:
        cmd.append("--release")
    for n in names:
        cmd.extend(["--bin", n])
    r = subprocess.run(cmd, cwd=workspace_root)
    return r.returncode == 0


def target_root(workspace_root: Path) -> Path:
    env = os.environ.get("CARGO_TARGET_DIR")
    return Path(env) if env else workspace_root / "target"


def binary_path(workspace_root: Path, name: str, release: bool) -> Path:
    profile = "release" if release else "debug"
    p = target_root(workspace_root) / profile / name
    if sys.platform == "win32":
        pe = p.with_suffix(".exe")
        if pe.is_file():
            return pe
    return p


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "problems",
        nargs="*",
        help=(
            "Cargo binary name(s). With --test-dir: exactly one. "
            "Otherwise: filter folders under test/ (default: all)"
        ),
    )
    parser.add_argument(
        "--no-build",
        action="store_true",
        help="Skip cargo build; use existing binaries in target/",
    )
    parser.add_argument(
        "--debug",
        action="store_true",
        help="Use debug build instead of release",
    )
    parser.add_argument(
        "--timeout",
        type=float,
        default=None,
        metavar="SEC",
        help="Per-test wall-clock limit (default: none)",
    )
    parser.add_argument(
        "-q",
        "--quiet",
        action="store_true",
        help="Only print failures and final summary",
    )
    parser.add_argument(
        "--test-dir",
        type=Path,
        default=None,
        metavar="DIR",
        help=(
            "Directory to scan recursively for *.in (or a parent of such folders). "
            "Relative paths are resolved from the potyczki crate directory. "
            "Requires exactly one positional binary name. Each .in uses <stem>.out "
            "in the same directory."
        ),
    )
    args = parser.parse_args()

    potyczki_root, workspace_root, default_test_root = repo_layout(Path(__file__))

    if args.test_dir is not None:
        if len(args.problems) != 1:
            parser.error(
                "--test-dir requires exactly one BINARY (positional), the cargo --bin to run"
            )
        td = Path(args.test_dir)
        scan_root = td if td.is_absolute() else (potyczki_root / td)
        if not scan_root.is_dir():
            print(f"Not a directory: {scan_root}", file=sys.stderr)
            return 2
        problems = [args.problems[0]]
        scan_roots: list[tuple[str, Path]] = [(problems[0], scan_root)]
    else:
        test_root = default_test_root
        problems = discover_problems(test_root, args.problems or None)
        if not problems:
            print(f"No test directories found under {test_root}/.")
            return 0
        scan_roots = [(prob, test_root / prob) for prob in problems]

    release = not args.debug
    if not args.no_build:
        print("Building:", ", ".join(problems))
        if not build_binaries(workspace_root, problems, release):
            print("cargo build failed.", file=sys.stderr)
            return 2

    total_pass = total_fail = 0
    quiet = args.quiet
    if not quiet:
        print()

    for prob, prob_dir in scan_roots:
        bin_path = binary_path(workspace_root, prob, release)
        if not bin_path.is_file():
            print(
                f"=== {prob} ===\n  SKIP: missing binary {bin_path} (build failed or wrong name?)\n"
            )
            continue

        in_files = sorted(prob_dir.rglob("*.in"))
        if not in_files and args.test_dir is not None:
            print(f"=== {prob} ===\n  no *.in files under {prob_dir}\n")
            continue

        pass_n = fail_n = 0
        skip_n = 0
        if quiet:
            print(f"=== {prob} ({len(in_files)} tests) ===", end="", flush=True)
        else:
            print(f"=== {prob} ({len(in_files)} tests) ===")

        for in_path in in_files:
            rel = in_path.relative_to(prob_dir)
            out_path = in_path.with_suffix(".out")
            label = str(rel)
            if not out_path.is_file():
                skip_n += 1
                if not quiet:
                    print(f"  SKIP {label}: missing {out_path.name}")
                continue

            ok, err, sec = run_one(bin_path, in_path, out_path, args.timeout)
            if ok:
                pass_n += 1
                if not quiet:
                    print(f"  OK   {label}  ({sec:.3f}s)")
            else:
                fail_n += 1
                if quiet:
                    print()
                print(f"  FAIL {label}  ({sec:.3f}s)")
                for line in err.splitlines():
                    print(f"         {line}")

        total_pass += pass_n
        total_fail += fail_n
        if quiet:
            print(f" → {pass_n} ok", end="")
            if fail_n:
                print(f", {fail_n} failed", end="")
            if skip_n:
                print(f", {skip_n} skipped", end="")
            print()
        else:
            print(f"  subtotal: {pass_n} passed, {fail_n} failed\n")

    print("---")
    print(f"TOTAL: {total_pass} passed, {total_fail} failed")
    return 1 if total_fail else 0


if __name__ == "__main__":
    raise SystemExit(main())
