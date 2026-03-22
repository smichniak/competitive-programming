//! LeetCode 7. Reverse Integer.
//!
//! https://leetcode.com/problems/reverse-integer/

pub struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let negative: bool = x < 0;
        let mut positive_x: i32 = x.abs();
        let mut result: Option<i32> = Some(0);

        while positive_x > 0 {
            result = result
                .and_then(|r| r.checked_mul(10))
                .and_then(|r| r.checked_add(positive_x % 10));
            positive_x /= 10;
        }

        result
            .and_then(|r| r.checked_mul(if negative { -1 } else { 1 }))
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let x: i32 = 123;
        let result: i32 = Solution::reverse(x);
        assert_eq!(result, 321);
    }

    #[test]
    fn example2() {
        let x: i32 = -123;
        let result: i32 = Solution::reverse(x);
        assert_eq!(result, -321);
    }

    #[test]
    fn example3() {
        let x: i32 = 120;
        let result: i32 = Solution::reverse(x);
        assert_eq!(result, 21);
    }
}
