use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

use leetcode_in_rust::questions::sequences::two_sum::*;

fn solve_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let mut group = c.benchmark_group("Two Sum");
    let array = (0..1000).map(|_| rng.gen_range(0..10)).collect::<Vec<_>>();
    for i in -2..=2 {
        group.bench_with_input(
            BenchmarkId::new(format!("O(n^2) target={}", i), i),
            &i,
            |b, i| b.iter(|| two_sum_naive(&array, *i)),
        );
        group.bench_with_input(
            BenchmarkId::new(format!("O(n) target={}", i), i),
            &i,
            |b, i| b.iter(|| two_sum(&array, *i)),
        );
    }
}

criterion_group!(benches, solve_benchmark);
criterion_main!(benches);
