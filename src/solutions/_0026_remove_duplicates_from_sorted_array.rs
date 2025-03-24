use crate::solutions::Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        *nums = nums.iter().fold(vec![], |mut acc, &num| {
            if acc.is_empty() || acc.last() != Some(&num) {
                acc.push(num);
            }

            acc
        });

        nums.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::remove_duplicates(&mut vec![1, 1, 2]), 2);
    assert_eq!(
        Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4]),
        5
    );
}
