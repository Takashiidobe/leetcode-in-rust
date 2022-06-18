use crate::*;
use std::collections::HashMap;

test! {
    test_1: two_sum(vec![2, 7, 11, 15], 9), (0, 1),
    test_2: two_sum(vec![3, 2, 4], 6), (1, 2),
    test_3: two_sum(vec![3, 3], 6), (0, 1),
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> (i32, i32) {
    let mut m = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if m.contains_key(num) {
            return (m[num], index as i32);
        }
        m.insert((target - num) as i32, index as i32);
    }

    (-1, -1)
}
