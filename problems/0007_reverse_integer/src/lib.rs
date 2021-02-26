/// NO. 7: Reverse Integer

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut raw = x;
        let mut rev = 0i32;
        while raw != 0 {
            let dig = raw % 10;
            let tmp = rev * 10 + dig;
            if (tmp - dig) / 10 != rev {
                return 0
            }
            raw /= 10;
            rev = tmp;
        }
        rev
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
    }
}
