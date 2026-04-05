//! LeetCode 657. Robot Return To Origin.
//!
//! https://leetcode.com/problems/robot-return-to-origin/

pub struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        moves.chars().fold((0, 0), |(x, y), m| match m {
            'U' => (x, y + 1),
            'D' => (x, y - 1),
            'L' => (x - 1, y),
            'R' => (x + 1, y),
            _ => unreachable!(),
        }) == (0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let moves = "UD";
        let result = Solution::judge_circle(moves.to_string());
        assert!(result);
    }

    #[test]
    fn example2() {
        let moves = "LL";
        let result = Solution::judge_circle(moves.to_string());
        assert!(!result);
    }
}
