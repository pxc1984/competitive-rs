use std::collections::VecDeque;

pub fn longest_palindrome(s: String) -> String {
    let mut queue = VecDeque::from([s.clone()]);
    let mut iter_count = 0;
    let len = s.len();
    let max_iter_count = (len / 2);
    while let Some(elem) = queue.pop_front() {
        if elem.len() == 0 || is_palindrome(&elem) {
            return elem;
        }
        if iter_count < max_iter_count {
            queue.push_back(s[..len - iter_count].to_string());
            queue.push_back(s[iter_count..].to_string());
            queue.push_back(s[iter_count..len - iter_count].to_string());
            iter_count += 1;
        }
    }
    return "".to_string();
}

fn is_palindrome(s: &String) -> bool {
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len / 2 {
        if bytes[i] != bytes[len - i - 1] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = [("babad", "bab"), ("cbbd", "bb")];

        for (input, expected) in test_cases {
            let out = longest_palindrome(input.to_string());
            assert_eq!(out.len(), expected.len());
        }
    }
}
