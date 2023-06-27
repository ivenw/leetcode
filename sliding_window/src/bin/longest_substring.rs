struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashSet;
        let mut set = HashSet::new();
        let mut l = 0;
        let mut res = 0;

        for r in 0..s.len() {
            while set.contains(&s[r..r]) {
                set.remove(&s[l..l]);
                l += 1;
            }
            set.insert(&s[r..r]);
            res = res.max(r - l + 1);
        }

        res as i32
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
