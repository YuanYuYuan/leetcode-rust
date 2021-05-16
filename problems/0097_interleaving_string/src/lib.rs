/// NO. 97: Interleaving String

pub struct Solution;

// ----- submission codes start here -----

use std::collections::HashSet;

impl Solution {
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        if s1.len() + s2.len() != s3.len() {
            false
        } else {
            let s1 = s1.as_bytes();
            let s2 = s2.as_bytes();
            let s3 = s3.as_bytes();
            let mut stack = vec![(0, 0)];
            let mut visited = HashSet::new();
            visited.insert((0, 0));

            while let Some((i, j)) = stack.pop() {
                if i + j == s3.len() {
                    return true
                }

                if i+1 <= s1.len() && !visited.contains(&(i+1, j)) && s1[i] == s3[i+j] {
                    stack.push((i+1, j));
                    visited.insert((i+1, j));
                }

                if j+1 <= s2.len() && !visited.contains(&(i, j+1)) && s2[j] == s3[i+j] {
                    stack.push((i, j+1));
                    visited.insert((i, j+1));
                }
            }
            false
        }
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbcbcac".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
    }

    #[test]
    fn test2() {
        let s1 = "aabcc".to_string();
        let s2 = "dbbca".to_string();
        let s3 = "aadbbbaccc".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), false);
    }

    #[test]
    fn test3() {
        let s1 = "".to_string();
        let s2 = "".to_string();
        let s3 = "".to_string();
        assert_eq!(Solution::is_interleave(s1, s2, s3), true);
    }
}
