struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut res = 0;
        let mut l = 0;

        for (r, _) in s.char_indices() {
            let window = &s[l..=r];
            let map = window
                .chars()
                .fold(HashMap::<char, i32>::new(), |mut acc, c| {
                    acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
                    acc
                });
            let non_major_char_count = map.values().sum::<i32>() - map.values().max().unwrap();
            if non_major_char_count <= k {
                res = res.max(window.len());
            } else {
                l += 1;
            }
        }

        res as i32
    }
}

fn main() {
    assert!(Solution::character_replacement("ABAB".to_string(), 2) == 4);
    assert!(Solution::character_replacement("AABABBA".to_string(), 1) == 4);
    assert!(Solution::character_replacement("AABABBA".to_string(), 0) == 2);
    assert!(Solution::character_replacement("AABABBA".to_string(), 3) == 7);
}
