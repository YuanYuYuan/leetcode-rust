/// NO. 16: 3Sum Closest

pub struct Solution;

// ----- submission codes start here -----

use std::cmp::Ordering::{Equal, Greater, Less};

impl Solution {
    pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
        let mut min_diff = std::i32::MAX;
        let mut nums = nums;
        nums.sort();
        for i in 0..(nums.len()-2) {
            let (mut j, mut k) = (i+1, nums.len()-1);
            while j < k {
                let diff = nums[i] + nums[j] + nums[k] - target;
                match diff.cmp(&0) {
                    Less => j += 1,
                    Equal => return target,
                    Greater => k -= 1,
                }
                if diff.abs() < min_diff.abs() {
                    min_diff = diff;
                }
            }
        }
        target + min_diff
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::three_sum_closest(vec![-1, 2, 1, -4], 1), 2);
    }
}
