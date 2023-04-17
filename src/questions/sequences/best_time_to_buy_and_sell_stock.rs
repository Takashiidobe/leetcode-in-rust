use crate::*;

test! {
    test_1: best_time_to_buy_and_sell_stock(&[7,1,5,3,6,4]), 5,
    test_2: best_time_to_buy_and_sell_stock(&[7,6,4,3,1]), 0,
    test_3: best_time_to_buy_and_sell_stock(&[1, 2]), 1,
}

pub fn best_time_to_buy_and_sell_stock(prices: &[i32]) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut min_so_far = prices[0];
    let mut max_profit = 0;

    for price in &prices[1..] {
        min_so_far = min_so_far.min(*price);
        max_profit = max_profit.max(price - min_so_far);
    }
    max_profit
}
