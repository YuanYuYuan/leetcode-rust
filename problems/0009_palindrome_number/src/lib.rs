/// NO. 9: Palindrome Number

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            false
        } else if x < 10 {
            true
        } else if x % 10 == 0 {
            false
        } else {
            let (mut x, mut rev) = (x, 0);
            while x > rev {
                rev = rev * 10 + x % 10;
                x /= 10;
            }
            // cases of even or odd
            x == rev || x == rev / 10
        }
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_palindrome(123), false);
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome(-121), false);
    }
}
