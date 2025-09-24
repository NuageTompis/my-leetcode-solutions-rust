struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut bitmap = [0; 26];
        let mut right = [0; 26];
        let mut left_bit_map = 1 << (s[0] - b'a');
        let mut right_bit_map = 0;

        for c in s.iter().take(n).skip(1) {
            let idx = (c - b'a') as usize;
            right[idx] += 1;
            right_bit_map |= 1 << idx;
        }
        for c in s.iter().take(n - 1).skip(1) {
            let idx = (c - b'a') as usize;
            right[idx] -= 1;
            if right[idx] == 0 {
                right_bit_map -= 1 << idx;
            }
            bitmap[idx] |= left_bit_map & right_bit_map;
            left_bit_map |= 1 << idx;
        }
        bitmap
            .into_iter()
            .map(|v| {
                let (mut v, mut c) = (v, 0);
                while v > 0 {
                    c += 1;
                    v &= v - 1;
                }
                c
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "aabca".into();
        let res = Solution::count_palindromic_subsequence(s);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "adc".into();
        let res = Solution::count_palindromic_subsequence(s);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "bbcbaba".into();
        let res = Solution::count_palindromic_subsequence(s);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }
}
