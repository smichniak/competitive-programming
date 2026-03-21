"""CLI: run tests for a LeetCode solution by number or slug."""

from __future__ import annotations

import argparse
import subprocess
import sys

from ._resolve import iter_problem_files, resolve_problem_file
from ._scaffold import write_new_problem


def _cmd_run(problem: str, pytest_args: list[str]) -> int:
    path = resolve_problem_file(problem)
    cmd = [sys.executable, "-m", "pytest", str(path), *pytest_args]
    return subprocess.call(cmd)


def _cmd_list() -> int:
    for p in iter_problem_files():
        print(p.stem)
    return 0


def _cmd_new(number: int, slug: str, *, overwrite: bool) -> int:
    try:
        path = write_new_problem(number, slug, overwrite=overwrite)
    except (ValueError, FileExistsError) as e:
        print(f"error: {e}", file=sys.stderr)
        return 1
    print(path)
    return 0


def main() -> None:
    parser = argparse.ArgumentParser(prog="python -m leetcode")
    sub = parser.add_subparsers(dest="command", required=True)

    run_p = sub.add_parser("run", help="Run pytest on one problem file")
    run_p.add_argument(
        "problem",
        help="Problem number (e.g. 25) or slug fragment (e.g. reverse_nodes)",
    )
    run_p.add_argument(
        "pytest_args",
        nargs=argparse.REMAINDER,
        help="Extra arguments passed to pytest (use after -- if needed)",
    )

    sub.add_parser("list", help="List all solution stems (NNNN_slug)")

    new_p = sub.add_parser(
        "new",
        help="Create NNNN_slug.py from problem number and slug (hyphen or snake_case)",
    )
    new_p.add_argument("number", type=int, help="LeetCode problem number (e.g. 1)")
    new_p.add_argument(
        "slug",
        help='Slug or title, e.g. two-sum or "two sum"',
    )
    new_p.add_argument(
        "--force",
        action="store_true",
        help="Overwrite if the file already exists",
    )

    args = parser.parse_args()
    if args.command == "run":
        extra = args.pytest_args
        if extra and extra[0] == "--":
            extra = extra[1:]
        raise SystemExit(_cmd_run(args.problem, extra))
    if args.command == "list":
        raise SystemExit(_cmd_list())
    if args.command == "new":
        raise SystemExit(_cmd_new(args.number, args.slug, overwrite=args.force))


if __name__ == "__main__":
    main()
