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
