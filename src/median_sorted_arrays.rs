pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut nums1 = nums1.clone();
    let mut nums2 = nums2.clone();
    nums1.append(&mut nums2);
    nums1.sort();
    median(nums1)
}

fn median(arr: Vec<i32>) -> f64 {
    if arr.len() % 2 == 0 {
        return (arr[arr.len() / 2 - 1] + arr[arr.len() / 2]) as f64 / 2.;
    } else {
        return arr[arr.len() / 2] as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_cases = [[vec![1, 3], vec![2]], [vec![1, 2], vec![3, 4]]];
        let test_expected = [2., 2.5];
        for (input, expected) in test_cases.iter().zip(test_expected) {
            let output = find_median_sorted_arrays(input[0].clone(), input[1].clone());
            assert_eq!(expected, output);
        }
    }
}
