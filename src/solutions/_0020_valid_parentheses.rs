use crate::solutions::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::with_capacity(s.len());

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.pop() != Some(c) {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }

        stack.is_empty()
    }
}

#[test]
fn test() {
    assert!(Solution::is_valid("()".to_owned()));
    assert!(Solution::is_valid("()[]{}".to_owned()));
    assert!(!Solution::is_valid("(]".to_owned()));
    assert!(Solution::is_valid("([])".to_owned()));
}
