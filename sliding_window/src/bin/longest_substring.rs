struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut map: HashMap<char, usize> = HashMap::new();
        let mut l = 0;
        let mut res = 0;

        for (r, c) in s.chars().enumerate() {
            // get previous position of char if it has been seen before
            if let Some(pos) = map.get(&c) {
                // is the previous position larger or equal to the left pointer
                if *pos >= l {
                    // mark of end of substring, so update largest length
                    res = res.max(r - l);
                    // move left pointer one to the right to the previous position of
                    // the char
                    l = *pos + 1;
                }
            }
            map.insert(c, r);
        }
        res.max(s.len() - l) as i32
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
