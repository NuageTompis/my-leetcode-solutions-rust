struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn resulting_string(s: String) -> String {
        let a_u8 = b'a';
        let s: Vec<u8> = s.as_bytes().iter().map(|c| c - a_u8).collect();
        let mut queue: VecDeque<u8> = VecDeque::new();

        for c in s {
            if let Some(prev) = queue.pop_back() {
                if (prev + 1) % 26 == c || (c + 1) % 26 == prev {
                    continue;
                } else {
                    queue.push_back(prev);
                }
            }
            queue.push_back(c);
        }

        let mut res = String::new();
        while let Some(c) = queue.pop_front() {
            res.push((c + a_u8) as char);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "abc".into();
        let res = Solution::resulting_string(s);
        let expected: String = "c".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "adcb".into();
        let res = Solution::resulting_string(s);
        let expected: String = "".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "zadb".into();
        let res = Solution::resulting_string(s);
        let expected: String = "db".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
