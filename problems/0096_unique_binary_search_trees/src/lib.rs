/// NO. 96: Unique Binary Search Trees

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    // Denote the number of trees with k nodes by m[k].
    // For a tree with 4 nodes, the number of possible trees
    // m[4] = m[0] x m[3] + m[1] x m[2] + m[2] x m[1] + m[3] x m[0],
    // which enumerates all combinations of left and right subtrees.
    pub fn num_trees(n: i32) -> i32 {
        let n = n as usize;
        // start from m[0] = m[1] = 1
        let num = (2..=n).fold(vec![1, 1], |mut m, n_nodes| {
            m.push((0..n_nodes).fold(0, |sum, i| sum + m[i] * m[n_nodes-1-i]));
            m
        });
        num[n]
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for (i, &v) in (0..10).zip(&[1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862]) {
            assert_eq!(Solution::num_trees(i), v);
        }
    }
}
