struct Solution;

impl Solution {
    pub fn minimum_steps(s: String) -> i64 {
        let mut total = 0;
        let mut whites = 0;
        for &c in s.as_bytes().iter().rev() {
            if c == b'1' {
                total += whites;
            } else {
                whites += 1;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "101".into();
        let res = Solution::minimum_steps(s);
        let expected: i64 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "100".into();
        let res = Solution::minimum_steps(s);
        let expected: i64 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "0111".into();
        let res = Solution::minimum_steps(s);
        let expected: i64 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
