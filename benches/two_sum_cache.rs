use leetcode_in_rust::questions::sequences::two_sum::*;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use static_init::dynamic;

use iai_callgrind::{black_box, main};

#[dynamic]
static mut L1: Vec<i32> = {
    let mut rng = ChaCha8Rng::seed_from_u64(0);
    (0..1000)
        .map(|_| rng.gen_range(-100..100))
        .collect::<Vec<_>>()
};

#[dynamic]
static mut L2: Vec<i32> = {
    let l1 = L1.read();
    l1.clone().to_vec()
};

#[dynamic]
static mut L3: Vec<i32> = {
    let l1 = L1.read();
    l1.clone().to_vec()
};

#[dynamic]
static mut L4: Vec<i32> = {
    let l1 = L1.read();
    l1.clone().to_vec()
};

#[inline(never)]
fn two_sum_cache() -> (i32, i32) {
    two_sum(&L1.read(), 200)
}

#[inline(never)]
fn two_sum_naive_cache() -> (i32, i32) {
    two_sum_naive(&L2.read(), 200)
}

#[inline(never)]
fn two_sum_sorted_cache() -> (i32, i32) {
    two_sum_sorted(&mut L3.write(), 200)
}

#[inline(never)]
fn two_sum_sorted_unstable_cache() -> (i32, i32) {
    two_sum_sorted_unstable(&mut L4.write(), 200)
}

iai_callgrind::main!(
    two_sum_cache,
    two_sum_sorted_unstable_cache,
    two_sum_sorted_cache,
    two_sum_naive_cache
);
