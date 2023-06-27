struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let (mut profit, mut price_payed) = (0, prices[0]);

        for price_offered in prices.into_iter().skip(1) {
            profit = profit.max(price_offered - price_payed);
            price_payed = price_payed.min(price_offered);
        }
        profit
    }
}
fn main() {
    assert!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]) == 5);
    assert!(Solution::max_profit(vec![7, 6, 4, 3, 1]) == 0);
    assert!(Solution::max_profit(vec![2, 2, 4, 1, 0]) == 2);
    assert!(Solution::max_profit(vec![1, 2, 3, 4, 5]) == 4);
}
