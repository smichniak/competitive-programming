"""LeetCode 3643. Flip Square Submatrix Vertically.

https://leetcode.com/problems/flip-square-submatrix-vertically/
"""

PROBLEM_NUMBER = 3643
PROBLEM_SLUG = "flip-square-submatrix-vertically"


class Solution:
    def reverseSubmatrix(
        self, grid: list[list[int]], x: int, y: int, k: int
    ) -> list[list[int]]:
        for i in range(x, x + k // 2):
            for j in range(y, y + k):
                target = x + k - 1 - i + x
                grid[i][j], grid[target][j] = grid[target][j], grid[i][j]
        return grid


def test_example1() -> None:
    grid = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12], [13, 14, 15, 16]]
    x = 1
    y = 0
    k = 3
    expected = [[1, 2, 3, 4], [13, 14, 15, 8], [9, 10, 11, 12], [5, 6, 7, 16]]
    assert Solution().reverseSubmatrix(grid, x, y, k) == expected


def test_example2() -> None:
    grid = [[3, 4, 2, 3], [2, 3, 4, 2]]
    x = 0
    y = 2
    k = 2
    expected = [[3, 4, 4, 2], [2, 3, 2, 3]]
    assert Solution().reverseSubmatrix(grid, x, y, k) == expected
