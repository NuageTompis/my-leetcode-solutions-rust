struct Solution;

impl Solution {
    pub fn can_be_valid(s: String, locked: String) -> bool {
        if s.len() % 2 == 1 {
            return false;
        }

        let mut r: (u32, u32) = (0, 0);
        for (b, l) in s.bytes().zip(locked.bytes()) {
            if l == b'0' {
                r = (r.0.saturating_sub(1), r.1 + 1);
            } else if b == b'(' {
                r = (r.0 + 1, r.1 + 1);
            } else if r.1 == 0 {
                return false;
            } else {
                r = (r.0.saturating_sub(1), r.1 - 1)
            }
        }

        r.0 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "))()))".into();
        let locked: String = "010100".into();
        let res = Solution::can_be_valid(s, locked);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "()()".into();
        let locked: String = "0000".into();
        let res = Solution::can_be_valid(s, locked);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = ")".into();
        let locked: String = "0".into();
        let res = Solution::can_be_valid(s, locked);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let s: String = "(((())(((())".into();
        let locked: String = "111111010111".into();
        let res = Solution::can_be_valid(s, locked);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
