struct Solution;

impl Solution {
    pub fn minimize_xor(num1: i32, mut num2: i32) -> i32 {
        let mut bits = 0;
        while num2 > 0 {
            if num2 & 1 > 0 {
                bits += 1;
            }
            num2 >>= 1;
        }

        let mut b = 1 << 30;

        while b > 0 && bits > 0 {
            if num1 & b > 0 {
                bits -= 1;
                if bits == 0 {
                    return ((!b) ^ (2 * b - 1)) & num1;
                }
            }
            b >>= 1;
        }

        b = 1;
        while bits > 0 {
            if num1 & b == 0 {
                bits -= 1;
            }
            b <<= 1;
        }

        num1 | (b - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num1: i32 = 3;
        let num2: i32 = 5;
        let res = Solution::minimize_xor(num1, num2);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let num1: i32 = 1;
        let num2: i32 = 12;
        let res = Solution::minimize_xor(num1, num2);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
