struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut chars = s.chars();
        let mut last = chars.next().unwrap();
        let mut sum = 0;
        for (i, c) in chars.enumerate() {
            if i % 2 == 0 && c != last {
                sum += 1;
            } else {
                last = c;
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
        let s: String = "1001".into();
        let res = Solution::min_changes(s);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "10".into();
        let res = Solution::min_changes(s);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "0000".into();
        let res = Solution::min_changes(s);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
