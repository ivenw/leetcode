struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_groups: HashMap<Vec<char>, Vec<String>> = HashMap::new();

        for s in strs.into_iter() {
            let mut s_sorted: Vec<char> = s.chars().collect();
            s_sorted.sort();

            anagram_groups.entry(s_sorted).or_insert(vec![]).push(s);
        }

        anagram_groups.into_values().collect()
    }
}

fn vecstr_to_vectstring(v: Vec<&str>) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(vecstr_to_vectstring(vec![
            "eat", "tea", "tan", "ate", "nat", "bat"
        ]))
    );
}
