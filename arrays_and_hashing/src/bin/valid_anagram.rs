struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut s_chars: Vec<char> = s.chars().collect();
        let mut t_chars: Vec<char> = t.chars().collect();
        s_chars.sort();
        t_chars.sort();
        s_chars == t_chars
    }
}

impl Solution {
    pub fn is_anagram_hash_map(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = HashMap::new();
        s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
        t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
        map.into_values().all(|v| v == 0)
    }
}

fn main() {
    assert!(Solution::is_anagram(
        "anagram".to_string(),
        "nagaram".to_string()
    ));
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));
    assert!(Solution::is_anagram("😋👌".to_string(), "👌😋".to_string()));
    assert!(Solution::is_anagram_hash_map(
        "anagram".to_string(),
        "nagaram".to_string()
    ));
    assert!(!Solution::is_anagram_hash_map(
        "rat".to_string(),
        "car".to_string()
    ));
}
