use crate::solutions::Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        !nums.windows(2).any(|window| window[0] % 2 == window[1] % 2)
    }
}

#[test]
fn test() {
    assert!(Solution::is_array_special(vec![1]));
    assert!(Solution::is_array_special(vec![2, 1, 4]));
    assert!(!Solution::is_array_special(vec![4, 3, 1, 6]));
}
