struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let mut length = 0;
        let mut start = 0;

        for end in 0..=s.len() {
            let substring = &s[start..end];
            let unique_chars_in_substring = substring.chars().collect::<HashSet<_>>();

            if substring.len() != unique_chars_in_substring.len() {
                start += 1;
            } else {
                length = length.max(substring.len());
            }
        }

        length as i32
    }
}

fn main() {
    assert!(Solution::length_of_longest_substring("abcabcbb".to_string()) == 3);
    assert!(Solution::length_of_longest_substring("abcabcbbabcd".to_string()) == 4);
    assert!(Solution::length_of_longest_substring("bbbbb".to_string()) == 1);
    assert!(Solution::length_of_longest_substring("pwwk".to_string()) == 2);
    assert!(Solution::length_of_longest_substring("pwwkew".to_string()) == 3);
    assert!(Solution::length_of_longest_substring("s".to_string()) == 1);
    assert!(Solution::length_of_longest_substring(" ".to_string()) == 1);
    assert!(Solution::length_of_longest_substring("dvdf".to_string()) == 3);
    assert!(Solution::length_of_longest_substring("dvadf".to_string()) == 4);
    assert!(Solution::length_of_longest_substring("".to_string()) == 0);
}
