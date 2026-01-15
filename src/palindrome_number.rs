pub fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let s = x.to_string();
    let bytes = s.as_bytes();
    let len = bytes.len();
    for i in 0..len / 2 {
        if bytes[i] != bytes[len - 1 - i] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [
            (121, true),
            (-121, false),
            (10, false),
            (12321, true),
            (0, true),
        ];

        for (input, expected) in test_cases {
            assert_eq!(is_palindrome(input), expected);
        }
    }
}
