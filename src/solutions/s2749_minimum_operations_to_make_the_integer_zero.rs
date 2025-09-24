struct Solution;

impl Solution {
    pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
        let mut n = 1_u32;
        let num2 = num2 as i64;
        let mut num1 = num1 as i64 - num2;
        // we substract num2 from num1 and see whether is can be made from a sum of enough powers of two
        while num1 > 0 {
            // num1 is the highest amount of powers of two (ie adding 1 num1 times)
            if n as i64 <= num1 {
                let low = num1.count_ones(); // lowest amount of powers of to that can add up to num1
                if low <= n {
                    return n as i32;
                }
            }
            n += 1;
            num1 -= num2;
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num1: i32 = 3;
        let num2: i32 = -2;
        let res = Solution::make_the_integer_zero(num1, num2);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let num1: i32 = 5;
        let num2: i32 = 7;
        let res = Solution::make_the_integer_zero(num1, num2);
        let expected: i32 = -1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let num1: i32 = 112577768;
        let num2: i32 = -501662198;
        let res = Solution::make_the_integer_zero(num1, num2);
        let expected: i32 = 16; // Fill in this value
        assert_eq!(res, expected);
    }
}
