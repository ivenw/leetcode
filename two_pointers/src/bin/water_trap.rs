struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut max_l, mut max_r) = (height.first().unwrap(), height.last().unwrap());
        let (mut l, mut r) = (0, height.len() - 1);
        let mut res = 0;

        while l < r {
            match max_l.cmp(max_r) {
                std::cmp::Ordering::Greater => {
                    r -= 1;
                    max_r = max_r.max(&height[r]);
                    res += max_r - height[r];
                }
                _ => {
                    l += 1;
                    max_l = max_l.max(&height[l]);
                    res += max_l - height[l];
                }
            }
        }

        res
    }
}

fn main() {
    assert!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6);
    assert!(Solution::trap(vec![4, 2, 0, 3, 2, 5]) == 9);
    assert!(Solution::trap(vec![4, 2, 3]) == 1);
}
