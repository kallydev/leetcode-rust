use crate::solutions::Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() || (x != 0 && x % 10 == 0) {
            return false;
        }

        let (mut x, mut half) = (x, 0);

        while x > half {
            half = half * 10 + (x % 10);
            x /= 10;
        }

        x == half || x == half / 10
    }
}

#[test]
fn test() {
    assert!(Solution::is_palindrome(121));
    assert!(!Solution::is_palindrome(-121));
    assert!(!Solution::is_palindrome(10));
}
