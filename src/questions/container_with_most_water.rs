use crate::*;
use std::cmp::{max, min};

test! {
    test_1: max_area(vec![1, 1]), 1,
    test_2: max_area(vec![1, 2, 1]), 2,
    test_3: max_area(vec![4, 3, 2, 1, 4]), 16,
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;

    let mut max_so_far = 0;

    while i < j {
        let left = height[i];
        let right = height[j];
        let area = ((j - i) as i32) * min(left, right);
        if left < right {
            max_so_far = max(area, max_so_far);
            i += 1;
        } else {
            max_so_far = max(area, max_so_far);
            j -= 1;
        }
    }

    max_so_far
}
