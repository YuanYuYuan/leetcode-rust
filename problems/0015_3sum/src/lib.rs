/// NO. 15: 3Sum

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 3 {
            return vec![]
        }

        let mut nums = nums;
        nums.sort();
        let mut triplets = vec![];

        for i in 0..(nums.len()-2) {
            if nums[i] > 0 { break; }
            if i > 0 && nums[i] == nums[i-1] { continue; }
            let (mut j, mut k) = (i+1, nums.len()-1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                if sum == 0 {
                    triplets.push(vec![nums[i], nums[j], nums[k]]);
                    while {j += 1; nums[j-1] == nums[j] && j < k} {}
                    while {k -= 1; nums[k+1] == nums[k] && j < k} {}
                } else if sum > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        triplets
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::three_sum(vec![1, 2, -2, -1]),
            vec![vec![]]
        );
        assert_eq!(
            Solution::three_sum(vec![-1, 0, 1, 2, -1, 4]),
            [
                [-1, 0, 1],
                [-1, -1, 2]
            ].iter().map(|v| v.to_vec()).collect::<Vec<_>>()
        );
        assert_eq!(
            Solution::three_sum(vec![]),
            vec![vec![]]
        );
        assert_eq!(
            Solution::three_sum(vec![0]),
            vec![vec![]]
        );
    }
}
