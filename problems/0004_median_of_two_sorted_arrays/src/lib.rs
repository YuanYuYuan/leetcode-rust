/// NO. 4: Median of Two Sorted Arrays

pub struct Solution;

// ----- submission codes start here -----

use std::i32;
use std::cmp::Ordering;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Assume that x is shorter than y
        let (x, y) = if nums1.len() < nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        let indices: Vec<usize> =(0..=x.len()).into_iter().collect();

        // We want to find some i1, i2 such that i1, i2 partition x, y into two left and right
        // parts with left x/y parts are less than right y/x parts. If so, the merged left parts
        // is guaranteed to be less than the merged right parts, and the medium would
        // occur arround the dividing elements.
        let get_partition = |i1: usize| -> (i32, i32, i32, i32) {
            let i2 = (x.len() + y.len() + 1) / 2 - i1;
            let left_x_max = if i1 == 0 { i32::MIN } else { x[i1-1] };
            let left_y_max = if i2 == 0 { i32::MIN } else { y[i2-1] };
            let right_x_min = if i1 >= x.len() { i32::MAX } else { x[i1] };
            let right_y_min = if i2 >= y.len() { i32::MAX } else { y[i2] };
            (left_x_max, left_y_max, right_x_min, right_y_min)
        };
        let i1 = indices.binary_search_by(|&i1| {
            let (lx_max, ly_max, rx_min, ry_min) = get_partition(i1);
            match (lx_max <= ry_min, ly_max <= rx_min) {
                (true, true) => Ordering::Equal,
                (true, false) => Ordering::Less,     // less than desired partition on x, move i1 right
                (false, true) => Ordering::Greater,  // greater than desired partition on x, move i1 left
                (false, false) => unreachable!(),
            }
        })
        .expect("Incorrect format!");

        let (lx_max, ly_max, rx_min, ry_min) = get_partition(i1);
        if (x.len() + y.len()) % 2 == 0 {
            (lx_max.max(ly_max) + rx_min.min(ry_min)) as f64 / 2.
        } else {
            lx_max.max(ly_max) as f64
        }
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(2.0, Solution::find_median_sorted_arrays(vec![1, 3], vec![2]));
    }

    #[test]
    fn test2() {
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]));
    }

    #[test]
    fn test3() {
        assert_eq!(2.5, Solution::find_median_sorted_arrays(vec![], vec![2, 3]));
    }

}
