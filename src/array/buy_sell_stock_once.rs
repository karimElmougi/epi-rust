use std::cmp::max;

pub fn buy_sell_stock_once(array: &[i32]) -> i32 {
    if array.is_empty() {
        return 0;
    }

    let mut minima = array.first().unwrap();
    let mut maxima = array.first().unwrap();
    let mut max_delta = 0;

    for price in array {
        if price < minima {
            max_delta = max(max_delta, maxima - minima);
            minima = price;
        }
        maxima = price;
    }

    max(max_delta, maxima - minima)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = vec![310, 315, 275, 295, 260, 270, 290, 230, 255, 250];
        let max_profit = buy_sell_stock_once(&a);
        assert_eq!(max_profit, 30);
    }
}
