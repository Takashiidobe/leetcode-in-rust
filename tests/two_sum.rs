use big_o_test::*;

use leetcode_in_rust::questions::sequences::two_sum::*;

#[test]
fn two_sum_test() {
    let i = 40000;
    let input1: Vec<_> = (-10000..10000).collect();
    let input1 = input1.leak();
    let input2: Vec<_> = (-20000..20000).collect();
    let input2 = input2.leak();
    test_algorithm(
        "Assert Two Sum is O(n) time, O(n) space",
        10,
        || {
            let input1: Vec<_> = (-10000..10000).collect();
            input1.leak();
            let input2: Vec<_> = (-20000..20000).collect();
            input2.leak();
        },
        20000,
        || {
            two_sum(input1, i);
            0
        },
        40000,
        || {
            two_sum(input2, i);
            0
        },
        BigOAlgorithmComplexity::ON,
        BigOAlgorithmComplexity::ON,
        &TimeUnits::MICROSECOND,
    )
}

#[test]
fn two_sum_sorted_test() {
    let i = 40000;
    let input1: Vec<_> = (-10000..10000).collect();
    let input1 = input1.leak();
    let input2: Vec<_> = (-20000..20000).collect();
    let input2 = input2.leak();
    test_algorithm(
        "Assert Two Sum Sorted is O(n log n) time, O(n) space",
        10,
        || {
            let input1: Vec<_> = (-10000..10000).collect();
            input1.leak();
            let input2: Vec<_> = (-20000..20000).collect();
            input2.leak();
        },
        20000,
        || {
            two_sum_sorted(input1, i);
            0
        },
        40000,
        || {
            two_sum_sorted(input2, i);
            0
        },
        BigOAlgorithmComplexity::ONLogN,
        BigOAlgorithmComplexity::ON,
        &TimeUnits::MICROSECOND,
    )
}

#[test]
fn two_sum_naive_test() {
    let i = 40000;
    let input1: Vec<_> = (-10000..10000).collect();
    let input1 = input1.leak();
    let input2: Vec<_> = (-20000..20000).collect();
    let input2 = input2.leak();
    test_algorithm(
        "Assert Two Sum Naive is O(n^2) time, O(1) space",
        10,
        || {
            let input1: Vec<_> = (-10000..10000).collect();
            input1.leak();
            let input2: Vec<_> = (-20000..20000).collect();
            input2.leak();
        },
        10000,
        || {
            two_sum_naive(input1, i);
            0
        },
        20000,
        || {
            two_sum_naive(input2, i);
            0
        },
        BigOAlgorithmComplexity::ON2,
        BigOAlgorithmComplexity::O1,
        &TimeUnits::MICROSECOND,
    )
}
