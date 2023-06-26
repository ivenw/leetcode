struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();

        let mut res = vec![];

        for (i, n_i) in nums.iter().enumerate() {
            if *n_i > 0 {
                break;
            }
            if i > 0 && *n_i == nums[i - 1] {
                continue;
            }

            let mut j = i + 1;
            let mut k = nums.len() - 1;
            while j < k {
                let n_j = nums[j];
                let n_k = nums[k];
                let sum = n_i + n_j + n_k;
                let triplet = vec![*n_i, n_j, n_k];

                match sum.cmp(&0) {
                    std::cmp::Ordering::Greater => k -= 1,
                    std::cmp::Ordering::Less => j += 1,
                    std::cmp::Ordering::Equal => {
                        res.push(triplet);
                        j += 1;
                        while nums[j] == nums[j - 1] && j < k {
                            j += 1;
                        }
                    }
                }
            }
        }
        dbg!(res)
    }
}

fn main() {
    assert!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]) == vec![vec![-1, -1, 2], vec![-1, 0, 1]]
    );
    assert!(Solution::three_sum(vec![0, 1, 1]) == Vec::<Vec<i32>>::new());
    assert!(Solution::three_sum(vec![0, 0, 0]) == vec![vec![0, 0, 0]]);
    assert!(Solution::three_sum(vec![0, 0, 0, 0]) == vec![vec![0, 0, 0]]);
}
