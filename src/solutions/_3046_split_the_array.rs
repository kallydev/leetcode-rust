use itertools::Itertools;

use crate::solutions::Solution;

impl Solution {
    pub fn is_possible_to_split(nums: Vec<i32>) -> bool {
        !nums.iter().counts().iter().any(|(_, &count)| count > 2)
    }
}

#[test]
fn test() {
    assert!(Solution::is_possible_to_split(vec![1, 1, 2, 2, 3, 4]));
    assert!(!Solution::is_possible_to_split(vec![1, 1, 1, 1]));
}
