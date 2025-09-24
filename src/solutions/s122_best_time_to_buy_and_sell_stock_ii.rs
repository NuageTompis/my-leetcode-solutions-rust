struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut sum = 0;
        for (i, p) in prices.iter().enumerate() {
            if i == prices.len() - 1 {
                continue;
            }
            if prices[i + 1] > *p {
                sum += prices[i + 1] - p;
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let prices: Vec<i32> = vec![7, 1, 5, 3, 6, 4];
        let res = Solution::max_profit(prices);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let prices: Vec<i32> = vec![1, 2, 3, 4, 5];
        let res = Solution::max_profit(prices);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let prices: Vec<i32> = vec![7, 6, 4, 3, 1];
        let res = Solution::max_profit(prices);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
