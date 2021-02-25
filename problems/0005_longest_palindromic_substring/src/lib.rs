/// NO. 5: Longest Palindromic Substring

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 { return s }

        let slice = s.as_bytes();
        let mut begin = 0;
        let mut length = 1;

        // For each center, e.g. e2, we scan (i, j) for two cases
        //     Even:     <-i  j->
        //         [ e1, (e2, e3), e4, e5, ... ]
        //     Odd:      <-i      j->
        //         [ e1, (e2, e3, e4), e5, ... ]
        for start_i in 0..slice.len() {
            for start_j in start_i..(start_i+2) {

                // find the longest palindrome (i, j)
                let (mut scan_i, mut scan_j) = (start_i, start_j);
                let (mut i, mut j) = (0, 0);
                while scan_j < slice.len() && slice[scan_i] == slice[scan_j] {
                    i = scan_i;
                    j = scan_j;

                    if scan_i == 0 {
                        break
                    } else {
                        scan_i -= 1;
                        scan_j += 1;
                    }
                }

                // Non-single element, check if find a longer one
                if i != j {
                    let len = j - i + 1;
                    if len > length {
                        begin = i;
                        length = len;
                    }
                }

            }
        }

        s[begin..begin+length].to_string()
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(Solution::longest_palindrome("babad".to_string()), "bab");
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
    }

    #[test]
    fn test4() {
        assert_eq!(Solution::longest_palindrome("ac".to_string()), "a");
    }

    #[test]
    fn test5() {
        assert_eq!(Solution::longest_palindrome("aacabdkacaa".to_string()), "aca");
    }

}
