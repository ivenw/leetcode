struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut filtered_s = s.to_ascii_lowercase();
        filtered_s.retain(|c| c.is_ascii_alphanumeric());

        filtered_s == filtered_s.chars().rev().collect::<String>()
    }
}

fn main() {
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_string()
    ));
    assert!(!Solution::is_palindrome("race a car".to_string()));
}
