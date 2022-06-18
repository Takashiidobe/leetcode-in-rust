use crate::*;

test! {
    test_1: coin_change(vec![1, 2, 5], 11), 3,
    test_2: coin_change(vec![2], 3), -1,
    test_3: coin_change(vec![1], 0), 0,
    test_4: coin_change(vec![1], 1), 1,
    test_5: coin_change(vec![1], 2), 2,
}

pub fn coin_change(coins: Vec<usize>, amount: usize) -> i32 {
    let mut dp = vec![-1; amount + 1];
    dp[0] = 0;

    for &coin in &coins {
        for i in coin..=amount {
            dp[i] = match dp[i - coin] {
                x if x != -1 && (dp[i] == -1 || dp[i] > x + 1) => x + 1,
                _ => dp[i],
            };
        }
    }

    dp[amount]
}
