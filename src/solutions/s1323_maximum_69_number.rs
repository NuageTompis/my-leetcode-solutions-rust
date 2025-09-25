struct Solution;

impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let stringified = num.to_string();
        for (i, &d) in stringified.as_bytes().iter().enumerate() {
            if d == b'6' {
                let to_add = 3 * 10_i32.pow((stringified.len() - 1 - i) as u32);
                return num + to_add;
            }
        }

        num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num: i32 = 9669;
        let res = Solution::maximum69_number(num);
        let expected: i32 = 9969; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let num: i32 = 9996;
        let res = Solution::maximum69_number(num);
        let expected: i32 = 9999; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let num: i32 = 9999;
        let res = Solution::maximum69_number(num);
        let expected: i32 = 9999; // Fill in this value
        assert_eq!(res, expected);
    }
}
