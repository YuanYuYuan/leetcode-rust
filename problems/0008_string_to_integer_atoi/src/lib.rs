/// NO. 8: String to Integer (atoi)

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chr = s.chars().skip_while(|x| x.is_whitespace()).peekable();
        let sgn = match chr.peek() {
            Some('+') => {
                chr.next();
                1
            }
            Some('-') => {
                chr.next();
                -1
            }
            _ => 1,
        };
        chr.into_iter()
            .take_while(|c| c.is_numeric())
            .try_fold(0i32, |v, c| {
                v.checked_mul(10)
                    .and_then(|v| v.checked_add(c.to_digit(10).unwrap() as i32))
            })
            .map(|v| v * sgn)
            .unwrap_or(if sgn > 0 {
                std::i32::MAX
            } else {
                std::i32::MIN
            })
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::my_atoi("42".to_string()), 42);
        assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
        assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
        assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    }
}
