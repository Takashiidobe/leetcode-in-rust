use iai_callgrind::{black_box, main};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn fib_cached(n: u64) -> u64 {
    if n == 0 || n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _ in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}

fn iai_benchmark_short() -> u64 {
    fib_cached(black_box(30))
}

fn iai_benchmark_long() -> u64 {
    fibonacci(black_box(30))
}

iai_callgrind::main!(iai_benchmark_short, iai_benchmark_long);
