/// NO. 11: Container With Most Water

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut i, mut j, mut max) = (0, height.len() - 1, 0);
        while i < j {
            let h = height[i].min(height[j]);
            max = max.max(h * (j - i) as i32);
            while height[i] <= h && i < j { i += 1; }
            while height[j] <= h && i < j { j -= 1; }
            // if height[i] < height[j] {
            //     i += 1;
            // } else {
            //     j -= 1;
            // }
        }
        max
    }
}

// ----- submission codes end here -----


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let vecs = [
            vec![1, 8, 6, 2, 5, 4, 8, 3, 7],
            vec![1, 1],
            vec![4, 3, 2, 1, 4],
            vec![1, 2, 1]
        ];
        let anss = [
            49,
            1,
            16,
            2
        ];

        for (vec, ans) in vecs.iter().cloned().zip(anss.iter()) {
            assert_eq!(Solution::max_area(vec), *ans);
        }
    }
}
