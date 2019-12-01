// https://leetcode.com/problems/two-sum/

use std::collections::HashMap;

#[allow(dead_code)]
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut result: Vec<i32> = vec![];

    for (index, n) in nums.iter().enumerate() {
        let index = index as i32;
        if map.contains_key(n) {
            result.push(*map.get(n).unwrap());
            result.push(index);
            break;
        }
        let key = target - n;
        map.insert(key, index);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = vec![0, 1];
        assert_eq!(two_sum(nums, target), result);
    }
}
