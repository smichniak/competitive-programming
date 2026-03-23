#!/usr/bin/env python3
"""
Run I/O tests under test/<binary_name>/.

For every *.in file (recursively), runs the matching Cargo binary with that
input and compares stdout to <same_stem>.out. Exit code 1 if any test fails.

Usage:
  ./run_tests.py              # all problems that have a folder under test/
  ./run_tests.py kon_b zmi_c  # only these binaries (if their test dirs exist)
  ./run_tests.py -q           # quiet: only failures + totals

Each directory test/<name>/ must match a Cargo [[bin]] name. All *.in files under
that tree are run; output is compared to the sibling file with extension .out.

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
            print(f"warning: no test directory test/{m}/", file=sys.stderr)
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
        help="Binary names to test (default: every folder under test/)",
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
    args = parser.parse_args()

    _, workspace_root, test_root = repo_layout(Path(__file__))
    problems = discover_problems(test_root, args.problems or None)
    if not problems:
        print("No test directories found under test/.")
        return 0

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

    for prob in problems:
        prob_dir = test_root / prob
        bin_path = binary_path(workspace_root, prob, release)
        if not bin_path.is_file():
            print(
                f"=== {prob} ===\n  SKIP: missing binary {bin_path} (build failed or wrong name?)\n"
            )
            continue

        in_files = sorted(prob_dir.rglob("*.in"))
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
