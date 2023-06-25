struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort();

        let mut previous_max_count = 1;
        let mut count = 1;
        let mut previous = nums[0];

        for n in nums {
            if n - previous == 1 {
                count += 1;
                previous = n;
            } else if n - previous == 0 {
                continue;
            } else {
                if count > previous_max_count {
                    previous_max_count = count;
                }
                count = 1;
                previous = n;
            }
        }
        previous_max_count.max(count)
    }
}

fn main() {
    assert!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]) == 4);
    assert!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]) == 9);
    assert!(Solution::longest_consecutive(vec![]) == 0);
    assert!(Solution::longest_consecutive(vec![-1, -1, 0, 1, 3, 4, 5, 6, 7, 8, 9]) == 7);
    assert!(Solution::longest_consecutive(vec![1, 2, 0, 1]) == 3);
}
