use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    for (count, iter) in nums.iter().enumerate() {
        hash_map.insert(*iter, count as i32);
    }

    let mut result_vec: Vec<i32> = Vec::new();
    for (count, iter) in nums.iter().enumerate() {
        let complement = target - iter;
        if hash_map.contains_key(&complement) && hash_map[&complement] != count as i32 {
            result_vec.push(count as i32);
            result_vec.push(hash_map[&complement]);
            break;
        }
    }

    return result_vec;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        println!("result = {:?}", result);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test_two_sum_2() {
        let nums = vec![2, 11, 11, 15];
        let target = 22;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }
}