use std::collections::VecDeque;

pub fn reverse(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let sign = n / n.abs(); // we'll multiply by it in the end
    let mut n = n.abs();

    let mut digits = VecDeque::new();
    while n > 0 {
        digits.push_back(n % 10);
        n /= 10;
    }
    let mut n = 0;
    let mut step = 0;
    while let Some(digit) = digits.pop_front() {
        if (n > i32::MAX / 10 || (n == i32::MAX / 10 && digit > i32::MAX % 10)) {
            //overflow will occur
            return 0;
        }
        n = n * 10 + digit;
    }
    return n * sign;
}

/*
2147483647
7463847412
2143443412
*/

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_reverse() {
        let test_cases = vec![
            (123, 321),
            (-123, -321),
            (2147483647, 0), // out of range
            (1534236469, 0),
            (0, 0),
        ];

        for (input, expected) in test_cases {
            assert_eq!(reverse(input), expected);
        }
    }
}
