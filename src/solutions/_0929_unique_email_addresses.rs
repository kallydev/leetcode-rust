use std::collections::HashSet;

use crate::solutions::Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut hash_set = HashSet::new();

        for email in emails {
            let (local, domain) = email.split_once("@").unwrap();

            let local = {
                let local = match local.split_once("+") {
                    Some((local, _)) => local.to_owned(),
                    None => local.to_owned(),
                };

                local.replace(".", "")
            };

            hash_set.insert(local.to_owned() + "@" + domain);
        }

        hash_set.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_owned(),
            "test.e.mail+bob.cathy@leetcode.com".to_owned(),
            "testemail+david@lee.tcode.com".to_owned(),
        ]),
        2
    );
    assert_eq!(
        Solution::num_unique_emails(vec![
            "a@leetcode.com".to_owned(),
            "b@leetcode.com".to_owned(),
            "c@leetcode.com".to_owned()
        ]),
        3
    );
    assert_eq!(
        Solution::num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_owned(),
            "test.email.leet+alex@code.com".to_owned(),
        ]),
        2
    );
}
