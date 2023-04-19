use rand::Rng;

use leetcode_in_rust::questions::sequences::contains_duplicate::*;

use criterion::{criterion_group, criterion_main, Criterion};

fn solve_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("Contains Duplicate");
    let array = (0..1000).map(|_| rng.gen_range(0..10)).collect::<Vec<_>>();
    group.bench_function("O(n)", |b| b.iter(|| contains_duplicate(&array)));
    group.bench_function("O(n log n)", |b| {
        b.iter(|| contains_duplicate_sort(&mut array.clone()))
    });
}

criterion_group!(benches, solve_benchmark);
criterion_main!(benches);
