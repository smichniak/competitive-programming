"""Resolve a problem query (number or slug) to a solution file path."""

from __future__ import annotations

import re
from pathlib import Path

_LEETCODE_DIR = Path(__file__).resolve().parent
PROBLEMS_DIR = _LEETCODE_DIR / "problems"
_STEM_RE = re.compile(r"^(\d{4})_(.+)\.py$")


def iter_problem_files() -> list[Path]:
    """All `NNNN_*.py` solution files under ``leetcode/problems/``."""
    return sorted(
        p
        for p in PROBLEMS_DIR.glob("[0-9][0-9][0-9][0-9]_*.py")
        if p.is_file()
    )


def _normalize_slug(s: str) -> str:
    return s.strip().lower().replace("-", "_")


def resolve_problem_file(query: str) -> Path:
    """
    Resolve `query` to a single solution file.

    - Numeric: ``25``, ``0025`` → ``0025_<slug>.py``
    - Slug: ``reverse_nodes``, ``reverse-nodes-in-k-group`` → substring match on
      the filename after the number prefix.
    """
    q = query.strip()
    if not q:
        raise ValueError("Empty problem query")

    if q.isdigit():
        padded = f"{int(q):04d}"
        matches = [p for p in iter_problem_files() if p.name.startswith(f"{padded}_")]
        if len(matches) == 1:
            return matches[0]
        if not matches:
            raise FileNotFoundError(
                f"No solution file for problem {int(q)} (expected {padded}_*.py under {PROBLEMS_DIR})"
            )
        raise FileNotFoundError(f"Multiple files for problem {int(q)}: {matches}")

    needle = _normalize_slug(q)
    candidates: list[Path] = []
    for p in iter_problem_files():
        m = _STEM_RE.match(p.name)
        if not m:
            continue
        slug = m.group(2).lower()
        if needle == slug or needle in slug:
            candidates.append(p)

    if len(candidates) == 1:
        return candidates[0]
    if not candidates:
        raise FileNotFoundError(
            f"No solution file matching slug {query!r} (files: {[p.name for p in iter_problem_files()]})"
        )
    raise FileNotFoundError(f"Ambiguous slug {query!r}, matches: {[c.name for c in candidates]}")
