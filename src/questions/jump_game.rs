use crate::*;

test! {
    test_1: can_jump(vec![2,3,1,1,4]), true,
    test_2: can_jump(vec![3,2,1,0,4]), false,
}

pub fn can_jump(nums: Vec<i32>) -> bool {
    nums.iter().enumerate().fold(0, |acc, (i, v)| {
        if acc < i as i32 {
            -1
        } else {
            acc.max(i as i32 + *v)
        }
    }) >= (nums.len() - 1) as i32
}
