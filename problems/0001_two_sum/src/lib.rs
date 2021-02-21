/// NO. 1: Two Sum

pub struct Solution;

// ----- submission codes start here -----

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // let mut hash_map = HashMap::with_capacity(nums.len());
        let mut hash_map = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            if let Some(&j) = hash_map.get(&(target - num)) {
                return vec![i as i32, j as i32]
            } else {
                hash_map.insert(num, i as i32);
            }
        }
        vec![]
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::two_sum(vec![2, 7, 11, 15], 9).sort(),
            vec![0, 1].sort(),
        );
        assert_eq!(
            Solution::two_sum(vec![3, 2, 4], 6).sort(),
            vec![1, 2].sort(),
        );
        assert_eq!(
            Solution::two_sum(vec![3, 3], 6).sort(),
            vec![0, 1].sort(),
        );
    }
}
