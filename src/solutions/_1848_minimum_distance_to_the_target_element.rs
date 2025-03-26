use crate::solutions::Solution;

impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        let mut min_distance = nums.len() as i32;

        for (i, &num) in nums.iter().enumerate() {
            if num == target {
                min_distance = min_distance.min((i as i32 - start).abs())
            }
        }

        min_distance
    }
}

#[test]
fn test() {
    assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
    assert_eq!(
        Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 1),
        0
    );
    assert_eq!(Solution::get_min_distance(vec![5, 3, 6], 5, 2), 2);
}
