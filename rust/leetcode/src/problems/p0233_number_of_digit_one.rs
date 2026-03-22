//! LeetCode 233. Number Of Digit One.
//!
//! https://leetcode.com/problems/number-of-digit-one/

pub struct Solution;

fn count_ones_from_digit_list(digits: Vec<u32>, ones_by_digit_number: &Vec<i32>) -> i32 {
    if digits.len() == 1 {
        return if digits[0] >= 1 {
            1
        } else {
            0
        }
    }

    let cutoff_count = count_ones_from_digit_list(digits[1..].to_vec(), ones_by_digit_number);
    let last_digit = digits[0];
    let digit_count = digits.len() - 1;

    let mut count = cutoff_count;

    for d in 0..last_digit {
        count += ones_by_digit_number[digit_count];
        if d == 1 {
            count += 10_i32.pow(digit_count as u32);
        }
    }

    if last_digit == 1 {
        count += digits[1..].iter().fold(0, |acc, &d| acc * 10 + d as i32) + 1;
    }

    count
}

impl Solution {
    pub fn count_digit_one(n: i32) -> i32 {
        let digits = n
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        let mut ones_by_digit_number = vec![0; digits.len() + 1];
        ones_by_digit_number[1] = 1;

        for i in 2..=digits.len() {
            ones_by_digit_number[i] = ones_by_digit_number[i - 1] * 10 + 10_i32.pow(i as u32 - 1);
        }

        count_ones_from_digit_list(digits, &ones_by_digit_number)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let n: i32 = 13;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 6);
    }

    #[test]
    fn example2() {
        let n: i32 = 0;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 0);
    }

    #[test]
    fn example3() {
        let n: i32 = 100;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 21);
    }

    #[test]
    fn example4() {
        let n: i32 = 200;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 140);
    }

    #[test]
    fn example5() {
        let n: i32 = 1234;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 689);
    }

    #[test]
    fn example6() {
        let n: i32 = 42690;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 27839);
    }

    #[test]
    fn example7() {
        let n: i32 = 416012;
        let result: i32 = Solution::count_digit_one(n);
        assert_eq!(result, 312818);
    }
}
