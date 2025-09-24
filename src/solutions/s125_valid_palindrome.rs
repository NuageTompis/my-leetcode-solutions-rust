struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s_u8 = s.as_bytes();
        let mut l = 0;
        let mut r = s_u8.len() - 1;
        while l < r {
            while l < r && !(s_u8[l] as char).is_alphanumeric() {
                l += 1;
            }
            while l < r && !(s_u8[r] as char).is_alphanumeric() {
                r -= 1;
            }
            if !(s_u8[r] as char).eq_ignore_ascii_case(&(s_u8[l] as char)) {
                return false;
            }
            l += 1;
            let (_, overflow) = r.overflowing_sub(1);
            if !overflow {
                r -= 1
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "A man, a plan, a canal: Panama".into();
        let res = Solution::is_palindrome(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "race a car".into();
        let res = Solution::is_palindrome(s);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = " ".into();
        let res = Solution::is_palindrome(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let s: String = "a.".into();
        let res = Solution::is_palindrome(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
