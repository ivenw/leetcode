// https://leetcode.com/problems/contains-duplicate/

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = std::collections::HashSet::new();
        for num in nums {
            if set.contains(&num) {
                return true;
            }
            set.insert(num);
        }
        false
    }
}

fn main() {
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    assert!(Solution::contains_duplicate(vec![
        1, 1, 1, 3, 3, 4, 3, 2, 4, 2
    ]));
}
