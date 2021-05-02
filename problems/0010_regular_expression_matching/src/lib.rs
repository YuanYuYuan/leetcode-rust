/// NO. 10: Regular Expression Matching

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let m = s.len();
        let n = p.len();
        let mut mat = vec![vec![false; n+1]; m+1];
        let text = s.as_bytes();
        let pattern = p.as_bytes();

        // 1st row
        mat[0][0] = true;
        for j in 0 .. n {
            if pattern[j] == b'*' {
                mat[0][j+1] = mat[0][j-1]
            }
        }

        for i in 0 .. m {
            for j in 0 .. n {
                if pattern[j] == b'.' || pattern[j] == text[i] {
                    mat[i+1][j+1] = mat[i][j];
                } else if pattern[j] == b'*' {
                    mat[i+1][j+1] = mat[i+1][j-1];
                    if pattern[j-1] == b'.' || pattern[j-1] == text[i] {
                        mat[i+1][j+1] |= mat[i][j+1];
                    }
                }

            }
        }

        mat[m][n]
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        for (s, p, ans) in &[
            ("aa", "a", false),
            ("aa", "a*", true),
            ("aab", "c*a*b", true),
            ("mississippi", "mis*is*p*.", false),
        ] {
            assert_eq!(
                Solution::is_match(s.to_string(), p.to_string()),
                *ans,
                "s: {}, p: {}", s, p
            );
        }
    }
}
