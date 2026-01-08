pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        let test_cases = [[vec![1, 3], vec![2]], [vec![1, 2], vec![3, 4]]];
        let test_expected = [2., 2.5];
        for (input, expected) in test_cases.iter().zip(test_expected) {
            let output = find_median_sorted_arrays(input[0].clone(), input[1].clone());
            assert_eq!(expected, output);
        }
    }
}
