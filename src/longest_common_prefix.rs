use std::iter::zip;
use std::str::Chars;

pub fn longest_common_prefix(s: Vec<String>) -> String {
    let first: Vec<char> = s.first().unwrap().chars().collect();

    let mut prefix_len = first.len();
    for other in s {
        prefix_len = prefix_len.min(
            first.iter()
                .zip(other.chars())
                .take_while(|(a, b)| *a == b)
                .count(),
        );
    }

    first[..prefix_len].iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let test_cases = [
            (vec!["hello", "hello, world"], "hello"),
            (vec!["dog", "racecar", "car"], ""),
        ];

        for (input, expected) in test_cases {
            let input = input.iter().map(|s| s.to_string()).collect();
            assert_eq!(expected, longest_common_prefix(input));
        }
    }
}