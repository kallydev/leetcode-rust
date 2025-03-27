use crate::solutions::Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = String::new();

        for c in s.chars() {
            if c.is_ascii_digit() {
                stack.pop();
            } else {
                stack.push(c);
            }
        }

        stack
    }
}

#[test]
fn test() {
    assert_eq!(Solution::clear_digits("abc".to_owned()), "abc");
    assert_eq!(Solution::clear_digits("cb34".to_owned()), "");
}
