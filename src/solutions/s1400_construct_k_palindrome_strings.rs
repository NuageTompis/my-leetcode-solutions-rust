struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        let mut occur = [0; 26];

        let a = b'a';
        for &c in s.as_bytes() {
            occur[(c - a) as usize] += 1;
        }

        let (pairs, singles) = occur
            .iter()
            .fold((0, 0), |acc, x| (acc.0 + x / 2, acc.1 + x % 2));

        if singles > k {
            return false;
        }

        singles + (pairs << 1) >= k
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "annabelle".into();
        let k: i32 = 2;
        let res = Solution::can_construct(s, k);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "leetcode".into();
        let k: i32 = 3;
        let res = Solution::can_construct(s, k);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "true".into();
        let k: i32 = 4;
        let res = Solution::can_construct(s, k);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
