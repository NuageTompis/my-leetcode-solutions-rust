struct Solution;

impl Solution {
    pub fn smallest_number(n: i32) -> i32 {
        let mut power = 1;
        while power - 1 < n {
            power <<= 1;
        }
        power - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 5;
        let res = Solution::smallest_number(n);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 10;
        let res = Solution::smallest_number(n);
        let expected: i32 = 15; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let n: i32 = 3;
        let res = Solution::smallest_number(n);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
