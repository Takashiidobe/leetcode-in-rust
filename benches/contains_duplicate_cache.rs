use rand::Rng;

use leetcode_in_rust::questions::sequences::contains_duplicate::*;

use iai_callgrind::{black_box, main};

fn input() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..1000)
        .map(|_| rng.gen_range(0..1000))
        .collect::<Vec<_>>()
}

fn contains_duplicate_cache() -> bool {
    contains_duplicate(black_box(&input()))
}

fn contains_duplicate_sort_cache() -> bool {
    let mut input = input();
    contains_duplicate_sort(black_box(&mut input))
}

iai_callgrind::main!(contains_duplicate_cache, contains_duplicate_sort_cache);
