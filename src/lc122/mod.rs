// 122. Best Time to Buy and Sell Stock II

#[cfg(test)]
mod tests;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut maxprofit = 0;
    for window in prices.windows(2) {
        maxprofit += 0.max(window[1] - window[0]);
    }
    maxprofit
}

// pub fn max_profit(prices: Vec<i32>) -> i32 {
//     let mut ans = 0;
//     for i in 1..prices.len() {
//         if prices[i] > prices[i - 1] {
//             ans += prices[i] - prices[i - 1];
//         }
//     }
//     ans
// }
