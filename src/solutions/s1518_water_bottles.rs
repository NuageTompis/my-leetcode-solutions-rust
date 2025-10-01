struct Solution;

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drank = num_bottles;
        let mut empty = num_bottles;
        while empty >= num_exchange {
            // refill
            let q = empty / num_exchange;
            empty %= num_exchange;
            // drink
            drank += q;
            empty += q;
        }

        drank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num_bottles: i32 = 9;
        let num_exchange: i32 = 3;
        let res = Solution::num_water_bottles(num_bottles, num_exchange);
        let expected: i32 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let num_bottles: i32 = 15;
        let num_exchange: i32 = 4;
        let res = Solution::num_water_bottles(num_bottles, num_exchange);
        let expected: i32 = 19; // Fill in this value
        assert_eq!(res, expected);
    }
}
