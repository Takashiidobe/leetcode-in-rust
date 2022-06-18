use crate::*;

use std::cmp::Ordering;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = vec![];
    nums.sort_unstable();

    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut l, mut r) = (i + 1, nums.len() - 1);
        while l < r {
            let (left, prev, right, num) = (nums[l], nums[l - 1], nums[r], nums[i]);

            if l > i + 1 && left == prev {
                l += 1;
                continue;
            }

            if r < nums.len() - 1 && right == nums[r + 1] {
                r -= 1;
                continue;
            }

            let total = left + right + num;

            match total.cmp(&0) {
                Ordering::Equal => {
                    res.push([num, left, right].to_vec());
                    l += 1;
                }
                Ordering::Less => {
                    l += 1;
                }
                Ordering::Greater => {
                    r -= 1;
                }
            }
        }
    }
    res
}

test! {
    test_1: three_sum(vec![-1,0,1,2,-1,-4]), vec![vec![-1,-1,2], vec![-1,0,1]],
    test_2: three_sum(vec![]), Vec::<Vec<i32>>::new(),
    test_3: three_sum(vec![0]), Vec::<Vec<i32>>::new(),
}
