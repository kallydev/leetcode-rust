use crate::solutions::Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let (mut left, mut right) = (0, height.len() - 1);
        let (mut left_max, mut right_max) = (0, 0);

        while left < right {
            left_max = left_max.max(height[left]);
            right_max = right_max.max(height[right]);

            if height[left] < height[right] {
                water += left_max - height[left];
                left += 1;
            } else {
                water += right_max - height[right];
                right -= 1;
            }
        }

        water
    }
}

#[test]
fn test() {
    assert_eq!(Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
    assert_eq!(Solution::trap(vec![4, 2, 0, 3, 2, 5]), 9);
}
