/// NO. 55: Jump Game

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut reach = 0;
        for (i, &v) in nums.iter().enumerate() {
            if i > reach { return false; }
            reach = reach.max(i + v as usize);
            if reach == nums.len()-1 { return true; }
        }
        true
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    }
}
