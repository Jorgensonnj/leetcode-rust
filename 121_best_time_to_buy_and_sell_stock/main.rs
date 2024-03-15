use std::time::Instant;

// problem
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock

// my original code
pub fn my_max_profit(prices: Vec<i32>) -> i32 {

    let mut profit = 0;
    let mut cost   = prices[0];
    for price in prices.iter() {
        cost = i32::min(*price, cost);
        profit = i32::max(profit, price-cost);
    }

    profit
}

//https://leetcode.com/problems/best-time-to-buy-and-sell-stock/solutions/1735433/rust-12-ms-2-9mb-oneliner
// someone else's code
pub fn other_max_profit(prices: Vec<i32>) -> i32 {
    prices
        .iter()
        .fold((0, i32::MAX), |(mut max_profit, mut cost), price| {
            cost = i32::min(*price, cost);
            max_profit = i32::max(max_profit, price-cost);
            (max_profit, cost)
        }).0
}

fn main() {

    let tests = vec![
        vec!(7,1,5,3,6,4),
        vec!(7,6,4,3,1),
        vec!(2,4,1)
    ];

    for ( index, test ) in tests.iter().enumerate() {

        let my_time = Instant::now();
        let my_result = my_max_profit( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, my_result, my_time.elapsed());

        let other_time = Instant::now();
        let other_result = other_max_profit( test.clone() );
        println!("Test: {}, Result: {:?}, Time: {:?}", index, other_result, other_time.elapsed());
    }
}
