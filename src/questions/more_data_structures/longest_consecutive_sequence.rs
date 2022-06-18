use crate::*;

use std::cmp::max;
use std::collections::HashSet;
use std::iter::FromIterator;

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut max_so_far = 0;
    let num_set: HashSet<i32> = HashSet::from_iter(nums);
    let mut curr_num = 1;
    let mut curr = 0;

    for num in &num_set {
        if !num_set.contains(&(num - 1)) {
            curr_num = *num;
            curr = 1;
        }

        while num_set.contains(&(curr_num + 1)) {
            curr_num += 1;
            curr += 1;
        }
        max_so_far = max(max_so_far, curr);
    }

    max_so_far
}

test! {
    test_1: longest_consecutive(vec![-7,-1,3,-9,-4,7,-3,2,4,9,4,-9,8,-7,5,-1,-7]), 4,
}
