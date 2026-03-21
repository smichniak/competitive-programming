//! LeetCode 3643. Flip Square Submatrix Vertically.
//!
//! https://leetcode.com/problems/flip-square-submatrix-vertically/

pub struct Solution;

impl Solution {
    pub fn reverse_submatrix(grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = grid.clone();
        for i in x..(x + k / 2) {
            for j in y..(y + k) {
                let i_usize = i as usize;
                let j_usize = j as usize;
                let target_index = (x + k - 1 - i + x) as usize;
                result[i_usize][j_usize] = grid[target_index][j_usize];
                result[target_index][j_usize] = grid[i_usize][j_usize];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let grid: Vec<Vec<i32>> = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        let x: i32 = 1;
        let y: i32 = 0;
        let k: i32 = 3;

        let result: Vec<Vec<i32>> = Solution::reverse_submatrix(grid, x, y, k);
        assert_eq!(
            result,
            vec![
                vec![1, 2, 3, 4],
                vec![13, 14, 15, 8],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 16]
            ]
        );
    }

    #[test]
    fn example2() {
        let grid: Vec<Vec<i32>> = vec![vec![3, 4, 2, 3], vec![2, 3, 4, 2]];
        let x: i32 = 0;
        let y: i32 = 2;
        let k: i32 = 2;

        let result: Vec<Vec<i32>> = Solution::reverse_submatrix(grid, x, y, k);
        assert_eq!(result, vec![vec![3, 4, 4, 2], vec![2, 3, 2, 3]]);
    }
}
