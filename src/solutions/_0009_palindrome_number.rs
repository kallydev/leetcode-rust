use crate::solutions::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let origin = x.to_string();
        let reserved = origin.chars().rev().collect::<String>();

        origin == reserved
    }
}

#[test]
fn test() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121));
    assert!(!Solution::is_palindrome(10));
}
