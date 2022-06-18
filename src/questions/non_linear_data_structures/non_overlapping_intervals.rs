use crate::*;

pub fn erase_overlap_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
    intervals.sort_by_key(|a| a[1]);
    let mut ans = 0;
    let mut end = i32::MIN;

    for interval in intervals {
        if interval[0] >= end {
            end = interval[1];
        } else {
            ans += 1;
        }
    }

    ans
}
