use std::collections::HashSet;

use crate::solutions::Solution;

impl Solution {
    pub fn repeated_character(s: String) -> char {
        let mut hash_set = HashSet::new();

        for c in s.chars() {
            if hash_set.contains(&c) {
                return c;
            }

            hash_set.insert(c);
        }

        unreachable!()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::repeated_character("abccbaacz".to_owned()), 'c');
    assert_eq!(Solution::repeated_character("abcdd".to_owned()), 'd');
}
