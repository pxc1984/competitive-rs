pub fn longest_palindrome(s: String) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_palindrome() {
        let test_cases = [("babad", "bab"), ("cbbd", "bb")];

        for (input, expected) in test_cases {
            let out = longest_palindrome(input.to_string());
            assert_eq!(out, expected.to_string());
        }
    }
}
