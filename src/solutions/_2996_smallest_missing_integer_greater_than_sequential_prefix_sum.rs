use std::collections::HashSet;

use crate::solutions::Solution;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let prefix_len = 1 + nums
            .windows(2)
            .take_while(|pair| pair[1] == pair[0] + 1)
            .count();

        let prefix_sum = nums.iter().take(prefix_len).sum::<i32>();

        let hash_set: HashSet<i32> = nums.iter().cloned().collect();

        (prefix_sum..)
            .find(|x| !hash_set.contains(x))
            .unwrap_or(prefix_sum)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::missing_integer(vec![1, 2, 3, 2, 5]), 6);
    assert_eq!(Solution::missing_integer(vec![3, 4, 5, 1, 12, 14, 13]), 15);
}
