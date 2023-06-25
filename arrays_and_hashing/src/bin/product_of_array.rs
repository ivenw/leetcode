struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = Vec::new();

        let mut prefix = 1;
        nums.iter().for_each(|n| {
            result.push(prefix);
            prefix *= n
        });

        let mut suffix = 1;
        result
            .iter_mut()
            .zip(nums.iter())
            .rev()
            .for_each(|(res, n)| {
                *res *= suffix;
                suffix *= n
            });

        result
    }
}

fn main() {
    assert!(Solution::product_except_self(vec![1, 2, 3, 4]) == vec![24, 12, 8, 6]);
    assert!(Solution::product_except_self(vec![-1, 1, 0, -3, 3]) == vec![0, 0, 9, 0, 0]);
}
