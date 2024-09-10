pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total_sum = nums.iter().sum::<i32>();

    let mut left_sum = 0;
    let mut right_sum = total_sum;
    let mut index = -1;

    for i in 0..nums.len() {
        left_sum += nums[i];

        if left_sum == right_sum {
            index = i as i32;
            break;
        }

        right_sum -= nums[i];
    }

    return index;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let nums = vec![1, 7, 3, 6, 5, 6];
        assert_eq!(pivot_index(nums), 3);
    }

    #[test]
    fn test_2() {
        let nums = vec![1, 2, 3];
        assert_eq!(pivot_index(nums), -1);
    }

    #[test]
    fn test_3() {
        let nums = vec![2, 1, -1];
        assert_eq!(pivot_index(nums), 0);
    }
}
