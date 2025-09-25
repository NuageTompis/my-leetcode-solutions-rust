struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut occur = [0; 26];
        let a = b'a';

        for &c in s.as_bytes() {
            occur[(c - a) as usize] += 1;
        }

        s.len() as i32
            - (occur
                .iter()
                .fold(0, |acc, &x| if x > 0 { acc + ((x - 1) >> 1) } else { acc })
                << 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "abaacbcbb".into();
        let res = Solution::minimum_length(s);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "aa".into();
        let res = Solution::minimum_length(s);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }
}
