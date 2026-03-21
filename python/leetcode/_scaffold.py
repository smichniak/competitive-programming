"""Create a new `NNNN_slug.py` solution file."""

from __future__ import annotations

import re
from pathlib import Path

from ._resolve import PROBLEMS_DIR


def _normalize_snake_slug(raw: str) -> str:
    s = raw.strip().lower().replace("-", "_")
    s = re.sub(r"[^a-z0-9_]+", "_", s)
    s = re.sub(r"_+", "_", s).strip("_")
    if not s:
        raise ValueError("Slug is empty after normalization")
    return s


def _snake_to_hyphen(snake: str) -> str:
    return snake.replace("_", "-")


def _slug_to_title(snake: str) -> str:
    parts = [p for p in snake.split("_") if p]
    return " ".join(p.capitalize() for p in parts)


def scaffold_path(number: int, slug_raw: str) -> Path:
    snake = _normalize_snake_slug(slug_raw)
    return PROBLEMS_DIR / f"{number:04d}_{snake}.py"


def render_new_file(number: int, slug_raw: str) -> str:
    snake = _normalize_snake_slug(slug_raw)
    hyphen = _snake_to_hyphen(snake)
    title = _slug_to_title(snake)
    return f'''"""LeetCode {number}. {title}.

https://leetcode.com/problems/{hyphen}/
"""

PROBLEM_NUMBER = {number}
PROBLEM_SLUG = "{hyphen}"


class Solution:
    pass


def test_stub() -> None:
    # Replace with real tests once you implement the solution.
    assert True
'''


def write_new_problem(number: int, slug_raw: str, *, overwrite: bool = False) -> Path:
    if number < 1:
        raise ValueError("Problem number must be positive")
    PROBLEMS_DIR.mkdir(parents=True, exist_ok=True)
    path = scaffold_path(number, slug_raw)
    if path.exists() and not overwrite:
        raise FileExistsError(f"File already exists: {path}")
    text = render_new_file(number, slug_raw)
    path.write_text(text, encoding="utf-8")
    return path
