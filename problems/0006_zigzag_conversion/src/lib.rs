/// NO. 6: ZigZag Conversion

pub struct Solution;

// ----- submission codes start here -----

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let n = num_rows as usize;
        let rows = (0..n).chain((1..(n - 1)).rev()).cycle();
        s.chars()
            .zip(rows)
            .fold(vec![String::new(); n], |mut vec, (ch, row)| {
                vec[row].push(ch);
                vec
            })
            .concat()
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 3),
            "PAHNAPLSIIGYIR",
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::convert("PAYPALISHIRING".to_string(), 4),
            "PINALSIGYAHRPI",
        );
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::convert("A".to_string(), 1), "A",);
    }
}
