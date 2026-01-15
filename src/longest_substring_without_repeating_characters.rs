use std::collections::HashSet;

pub fn length_of_longest_substring(str: String) -> i32 {
    println!("test case {str}");
    let mut max = 0;
    let mut counter = 0;

    let mut met = HashSet::new();
    for c in str.chars() {
        if let Some(_) = met.get(&c) {
            max = max.max(counter);
            println!("Counter reset. New max: {max}");
            counter = 0;
            met.clear();
        } else {
            met.insert(c.clone());
            println!("Met {c}");
            counter += 1;
        }
    }
    max = max.max(counter);

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let inputs = ["abcabcbb", "bbbbb", "pwwkew"];
        let outputs = [3, 1, 3];
        for (&i, o) in inputs.iter().zip(outputs) {
            assert_eq!(length_of_longest_substring(i.to_string()), o)
        }
    }
}
