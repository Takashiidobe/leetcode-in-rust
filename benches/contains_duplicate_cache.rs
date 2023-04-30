use leetcode_in_rust::questions::sequences::contains_duplicate::*;

use iai_callgrind::{black_box, main};
use parking_lot::RwLock;

use static_init::dynamic;

#[dynamic]
static INPUT: RwLock<Vec<i32>> = RwLock::new((0..100_000).collect::<Vec<_>>());

fn contains_duplicate_cache() -> bool {
    contains_duplicate(black_box(&INPUT.read()))
}

fn contains_duplicate_fnv_cache() -> bool {
    contains_duplicate_fnv(black_box(&INPUT.read()))
}

fn contains_duplicate_sort_cache() -> bool {
    contains_duplicate_sort(black_box(&mut INPUT.write()))
}

fn contains_duplicate_sort_unstable_cache() -> bool {
    contains_duplicate_sort_unstable(black_box(&mut INPUT.write()))
}

iai_callgrind::main!(
    contains_duplicate_cache,
    contains_duplicate_fnv_cache,
    contains_duplicate_sort_cache,
    contains_duplicate_sort_unstable_cache
);
