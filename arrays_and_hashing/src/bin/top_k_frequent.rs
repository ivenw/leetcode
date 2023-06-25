struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut count_map: HashMap<i32, u32> = HashMap::new();

        nums.iter()
            .for_each(|e| *count_map.entry(*e).or_insert(0) += 1);

        let mut sorted_counts: Vec<(i32, u32)> = count_map.into_iter().collect();
        sorted_counts.sort_by(|a, b| b.1.cmp(&a.1));

        let mut result = Vec::new();
        for (i, (n, _)) in sorted_counts.into_iter().enumerate() {
            result.push(n);
            if i as i32 == k - 1 {
                break;
            }
        }

        result
    }
}

fn main() {
    // let result = ;
    // println!("{result:?}");
    assert!(Solution::top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2) == vec![1, 2]);
}
