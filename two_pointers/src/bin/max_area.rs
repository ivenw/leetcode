struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_area = 0;
        let mut left_height = height[left];
        let mut right_height = height[right];

        while left < right {
            let min_height = left_height.min(right_height);
            let distance = right - left;
            let area = min_height * distance as i32;
            max_area = max_area.max(area);
            if left_height > right_height {
                right -= 1;
                right_height = height[right]
            } else {
                left += 1;
                left_height = height[left]
            }
        }

        max_area
    }
}

fn main() {
    assert!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49);
    assert!(Solution::max_area(vec![1, 1]) == 1);
}
