struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while i < j {
            let sum = numbers[i] + numbers[j];
            match sum.cmp(&target) {
                std::cmp::Ordering::Less => {
                    i += 1;
                }
                std::cmp::Ordering::Greater => {
                    j -= 1;
                }
                std::cmp::Ordering::Equal => {
                    return vec![i as i32 + 1, j as i32 + 1];
                }
            }
        }
        unreachable!()
    }
}

fn main() {
    assert!(Solution::two_sum(vec![2, 7, 11, 15], 9) == vec![1, 2]);
    assert!(Solution::two_sum(vec![2, 3, 4], 6) == vec![1, 3]);
    assert!(Solution::two_sum(vec![-1, 0], -1) == vec![1, 2]);
}
