use crate::*;
use std::cmp::{max, min};

test! {
    test_1: insert_interval(vec![vec![1, 3], vec![6, 8]], vec![2, 5]), vec![vec![1, 5], vec![6, 8]],
    test_2: insert_interval(vec![vec![]], vec![1,2]), vec![vec![1, 2]],
    test_3: insert_interval(vec![vec![]], vec![]), vec![vec![]],
}

pub fn insert_interval(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    if intervals.iter().all(|x| x.is_empty()) {
        return vec![new_interval];
    }
    if new_interval.is_empty() {
        return intervals;
    }
    let mut left = vec![];
    let mut mid = vec![new_interval[0], new_interval[1]];
    let mut right = vec![];

    for interval in intervals {
        if interval[1] < new_interval[0] {
            left.push(interval);
        } else if interval[0] > new_interval[1] {
            right.push(interval);
        } else {
            mid = vec![min(interval[0], mid[0]), max(interval[1], mid[1])];
        }
    }
    left.extend(vec![mid]);
    left.extend(right);
    left
}
