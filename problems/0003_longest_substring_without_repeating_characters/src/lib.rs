/// NO. 3: Longest Substring Without Repeating Characters

pub struct Solution;

// ----- submission codes start here -----

use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() == 0 {
            0
        } else {
            // Build a HashMap with capacity less than 128 since the key is only an usual character
            let mut hash_map: HashMap<char, usize> = HashMap::with_capacity(128.min(s.len()));
            // The desired length of the longest substring
            let mut length = 0;
            // Start of the string without repeating characters
            let mut start = 0;

            s.char_indices().into_iter().for_each(|(idx, c)| {
                if let Some(prev) = hash_map.get_mut(&c) {
                    start = start.max(*prev + 1);  // update start to a valid position
                    *prev = idx;
                } else {
                    hash_map.insert(c, idx);
                }
                length = length.max(idx - start + 1);
            });
            // Includes the possible length between the end and the start
            length.max(s.len() - start) as i32
        }
    }
}

//  ----- submission codes end here -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
    }

    #[test]
    fn test2() {
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
    }

    #[test]
    fn test4() {
        assert_eq!(
            Solution::length_of_longest_substring("".to_string()),
            0
        );
    }
}
