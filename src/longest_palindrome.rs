use std::collections::VecDeque;

pub fn longest_palindrome(s: String) -> String {
    if s.is_empty() {
        return String::new();
    }

    let bytes = s.as_bytes();
    let len = bytes.len();
    let (mut start, mut end) = (0, 0);

    for i in 0..len {
        let len1 = expand_around_center(bytes, i, i);
        let len2 = expand_around_center(bytes, i, i + 1);

        let max_len = len1.max(len2);

        if max_len > end - start {
            println!("{start} {end} {max_len} {len} {i}");
            start = i - (max_len - 1) / 2;
            end = i + max_len / 2;
        }
    }

    s[start..=end].to_string()
}

fn expand_around_center(bytes: &[u8], left: usize, right: usize) -> usize {
    let mut l = left as i32;
    let mut r = right as i32;
    let len = bytes.len() as i32;

    while l >= 0 && r < len && bytes[l as usize] == bytes[r as usize] {
        l -= 1;
        r += 1;
    }

    (r - l - 1) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            ("babad", "bab"),
            ("cbbd", "bb"),
            ("ac", "a"),
            ("a", "a"),
            ("abbcccba", "bcccb"),
            ("abb", "bb"),
        ];

        for (n, (input, expected)) in test_cases.iter().enumerate() {
            println!("Test {n} started");
            let out = longest_palindrome(input.to_string());
            assert_eq!(
                out.len(),
                expected.len(),
                "out: \"{out}\", expected: \"{expected}\""
            );
            println!("Test {n} successfull");
        }
    }
}
