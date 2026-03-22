//! LeetCode 1886. Determine Whether Matrix Can Be Obtained By Rotation.
//!
//! https://leetcode.com/problems/determine-whether-matrix-can-be-obtained-by-rotation/

pub struct Solution;

fn transpose(mat: &mut [Vec<i32>]) {
    for i in 0..mat.len() {
        let (upper, lower) = mat.split_at_mut(i);
        let row_i = &mut lower[0];
        for (j, row_j) in upper.iter_mut().enumerate() {
            std::mem::swap(&mut row_j[i], &mut row_i[j]);
        }
    }
}

fn reverse_rows(mat: &mut [Vec<i32>]) {
    for row in mat.iter_mut() {
        row.reverse();
    }
}

fn rotate(mat: &mut [Vec<i32>]) {
    transpose(mat);
    reverse_rows(mat);
}

impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        if mat == target {
            return true;
        }

        let mut local_mat = mat.clone();
        for _ in 0..3 {
            rotate(&mut local_mat);
            if local_mat == target {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let mat: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0]];
        let target: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
        assert!(Solution::find_rotation(mat, target));
    }

    #[test]
    fn example2() {
        let mat: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 1]];
        let target: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
        assert!(!Solution::find_rotation(mat, target));
    }

    #[test]
    fn example3() {
        let mat: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
        let target: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]];
        assert!(Solution::find_rotation(mat, target));
    }
}
