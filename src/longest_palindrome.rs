use std::collections::VecDeque;

pub fn longest_palindrome(s: String) -> String {
    let mut queue = populate_queue(&s);
    while let Some(elem) = queue.pop_front() {
        if elem.len() == 1 || is_palindrome(&elem) {
            return elem;
        }
    }
    return "".to_string();
}

fn populate_queue(s: &String) -> VecDeque<String> {
    let mut vec = Vec::new();
    let len = s.len();
    vec.push(s.clone());
    for i in 0..s.len() {
        for j in (i..=s.len()).rev() {
            if i == j {
                continue;
            }
            vec.push(s[i..j].to_string());
        }
    }
    vec.sort_by_key(|x| x.len());
    vec.reverse();
    VecDeque::from(vec)
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
        let test_cases = [
            ("babad", "bab"),
            ("cbbd", "bb"),
            ("ac", "a"),
            ("a", "a"),
            ("abbcccba", "bcccb"),
            ("abb", "bb"),
        ];

        for (input, expected) in test_cases {
            let out = longest_palindrome(input.to_string());
            assert_eq!(
                out.len(),
                expected.len(),
                "out: \"{out}\", expected: \"{expected}\""
            );
        }
    }
}
