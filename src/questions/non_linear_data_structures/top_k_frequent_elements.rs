use crate::*;

use std::collections::BinaryHeap;
use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for num in nums {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut v = Vec::new();

    for (key, val) in map {
        v.push((val, key));
    }

    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::from(v);

    let mut ans = Vec::new();
    (0..k).for_each(|_| ans.push(heap.pop().unwrap().1));
    ans
}

pub fn top_k_frequent_bucket_sort(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut cc: HashMap<i32, usize> = HashMap::new();
    let num_len = &nums.len();
    for n in nums {
        *cc.entry(n).or_insert(0) += 1;
    }
    let mut buckets: Vec<Vec<i32>> = vec![vec![]; num_len + 1];
    for (k, v) in cc {
        buckets[v].push(k);
    }
    let ans = buckets
        .into_iter()
        .flat_map(|b| b.into_iter())
        .collect::<Vec<i32>>();
    ans[ans.len() - k as usize..].to_vec()
}

test! {
    test_1: top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2), vec![1, 2],
    test_2: top_k_frequent(vec![1], 1), vec![1],
    test_3: top_k_frequent(vec![1,1,1,2,2,3], 3), vec![1,2,3],
}
