use fnv::FnvHashMap;

use crate::*;
use std::collections::HashMap;

test! {
    test_1: two_sum(&[2, 7, 11, 15], 9), (0, 1),
    test_2: two_sum(&[3, 2, 4], 6), (1, 2),
    test_3: two_sum(&[3, 3], 6), (0, 1),
}

pub fn two_sum(nums: &[i32], target: i32) -> (i32, i32) {
    let mut m: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for (index, num) in nums.iter().enumerate() {
        if let Some(i) = m.get(num) {
            return (*i, index as i32);
        } else {
            m.insert(target - num, index as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_bumpalo(nums: &[i32], target: i32) -> (i32, i32) {
    let mut m: HashMap<i32, i32> = HashMap::with_capacity(nums.len());
    for (index, num) in nums.iter().enumerate() {
        if let Some(i) = m.get(num) {
            return (*i, index as i32);
        } else {
            m.insert(target - num, index as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_fnv(nums: &[i32], target: i32) -> (i32, i32) {
    let mut m: FnvHashMap<i32, i32> = FnvHashMap::default();
    for (index, num) in nums.iter().enumerate() {
        if let Some(i) = m.get(num) {
            return (*i, index as i32);
        } else {
            m.insert(target - num, index as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_no_hash(nums: &[i32], target: i32) -> (i32, i32) {
    use nohash_hasher::BuildNoHashHasher;

    let mut m: HashMap<i32, i32, BuildNoHashHasher<i32>> =
        HashMap::with_capacity_and_hasher(nums.len(), BuildNoHashHasher::default());
    for (index, num) in nums.iter().enumerate() {
        if let Some(i) = m.get(num) {
            return (*i, index as i32);
        } else {
            m.insert(target - num, index as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_sorted(nums: &mut [i32], target: i32) -> (i32, i32) {
    nums.sort();
    for (i, num) in nums.iter().enumerate() {
        let num_to_get = target - num;
        if let Ok(j) = nums.binary_search(&num_to_get) {
            return (i as i32, j as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_sorted_unstable(nums: &mut [i32], target: i32) -> (i32, i32) {
    nums.sort_unstable();
    for (i, num) in nums.iter().enumerate() {
        let num_to_get = target - num;
        if let Ok(j) = nums.binary_search(&num_to_get) {
            return (i as i32, j as i32);
        }
    }

    (-1, -1)
}

pub fn two_sum_naive(nums: &[i32], target: i32) -> (i32, i32) {
    for i in 0..nums.len() - 1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return (i as i32, j as i32);
            }
        }
    }

    (-1, -1)
}
