use rand::{Rng, SeedableRng};

use leetcode_in_rust::questions::sequences::contains_duplicate::*;

use criterion::{criterion_group, criterion_main, Criterion};
use rand_chacha::ChaCha8Rng;
use static_init::dynamic;

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

fn solve_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Contains Duplicate");
    group.bench_function("O(n) HashSet", |b| {
        b.iter(|| contains_duplicate(&L1.read()))
    });
    group.bench_function("O(n) FNV HashSet", |b| {
        b.iter(|| contains_duplicate_fnv(&L1.read()))
    });
    group.bench_function("O(n log n) Sort", |b| {
        b.iter(|| contains_duplicate_sort_unstable(&mut L2.write()))
    });
    group.bench_function("O(n log n) Sort Unstable", |b| {
        b.iter(|| contains_duplicate_sort(&mut L3.write()))
    });
}

criterion_group!(benches, solve_benchmark);
criterion_main!(benches);
