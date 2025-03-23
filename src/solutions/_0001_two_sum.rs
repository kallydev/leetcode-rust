use std::collections::HashMap;

use crate::solutions::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.into_iter().enumerate() {
            if let Some(j) = hash_map.get(&(target - num)) {
                return vec![*j, i as i32];
            }

            hash_map.insert(num, i as i32);
        }

        unreachable!()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
}
