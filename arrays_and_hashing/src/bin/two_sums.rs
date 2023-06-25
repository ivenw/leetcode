struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if map.contains_key(&complement) {
                return vec![map[&complement], i as i32];
            }
            map.insert(*num, i as i32);
        }
        vec![]
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1])
}
