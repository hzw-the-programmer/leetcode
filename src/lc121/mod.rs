// 121. Best Time to Buy and Sell Stock

#[cfg(test)]
mod tests;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut minprice = i32::MAX;
    let mut maxprofit = 0;
    for price in prices {
        if price < minprice {
            minprice = price;
        } else if price - minprice > maxprofit {
            maxprofit = price - minprice;
        }
    }
    maxprofit
}
