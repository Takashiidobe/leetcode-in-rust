use crate::*;
use std::collections::HashSet;

test! {
    test_1: contains_duplicate(&[1, 2, 3, 1]), true,
    test_2: contains_duplicate(&[1, 2, 3, 4]), false,
    test_3: contains_duplicate(&[1]), false,
}

/// Returns `true` if nums contains a duplicate, `false otherwise.`
pub fn contains_duplicate(nums: &[i32]) -> bool {
    let s: HashSet<&i32> = HashSet::from_iter(nums.iter());
    s.len() != nums.len()
}

pub fn contains_duplicate_sort(nums: &mut [i32]) -> bool {
    let len = nums.len();
    if nums.len() < 2 {
        return false;
    }
    nums.sort();

    let mut i = 0;
    let mut j = 1;
    while j < len {
        let (left, right) = (nums[i], nums[j]);
        if left == right {
            return true;
        }
        i += 1;
        j += 1;
    }
    false
}
