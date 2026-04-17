//! LeetCode 3761. Minimum Absolute Distance Between Mirror Pairs.
//!
//! https://leetcode.com/problems/minimum-absolute-distance-between-mirror-pairs/

use std::collections::HashMap;

fn num_reverse(num: i32, acc: i32) -> i32 {
    match num {
        0 => acc,
        n => num_reverse(n / 10, 10 * acc + n % 10),
    }
}

pub struct Solution;

impl Solution {
    pub fn min_mirror_pair_distance(nums: Vec<i32>) -> i32 {
        nums.iter()
            .enumerate()
            .scan(HashMap::new(), |map, (i, &x)| {
                let distance = Some(map.get(&x).map(|last_index| (i - last_index) as i32));
                map.insert(num_reverse(x, 0), i);
                distance
            })
            .flatten()
            .min()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        let nums = vec![12, 21, 45, 33, 54];
        let result = Solution::min_mirror_pair_distance(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example2() {
        let nums = vec![120, 21];
        let result = Solution::min_mirror_pair_distance(nums);
        assert_eq!(result, 1);
    }

    #[test]
    fn example3() {
        let nums = vec![21, 120];
        let result = Solution::min_mirror_pair_distance(nums);
        assert_eq!(result, -1);
    }
}
