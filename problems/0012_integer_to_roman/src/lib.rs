/// NO. 12: Integer to Roman

pub struct Solution;

// ----- submission codes start here -----

const DICT: [(i32, &str); 13] = [
    (1000, "M"),
    (900,  "CM"),
    (500,  "D"),
    (400,  "CD"),
    (100,  "C"),
    (90,   "XC"),
    (50,   "L"),
    (40,   "XL"),
    (10,   "X"),
    (9,    "IX"),
    (5,    "V"),
    (4,    "IV"),
    (1,    "I")
];

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        DICT.iter()
            .fold(
                (num, String::from("")),
                |(res, rom), (n, r)| (
                    res % n,
                    rom + &r.repeat((res / n) as usize)
                )
            ).1
    }
}

// ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::int_to_roman(3), "III");
        assert_eq!(Solution::int_to_roman(4), "IV");
        assert_eq!(Solution::int_to_roman(9), "IX");
        assert_eq!(Solution::int_to_roman(58), "LVIII");
        assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
    }
}
