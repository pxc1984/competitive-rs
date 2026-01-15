use std::{collections::HashMap, vec};

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, n) in nums.iter().enumerate() {
        if let Some(&k) = map.get(n) {
            return vec![k as i32, i as i32];
        } else {
            map.insert(target - n, i);
        }
    }
    return vec![];
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn test() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
