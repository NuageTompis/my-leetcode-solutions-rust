struct Solution;

impl Solution {
    pub fn calculate_score(s: String) -> i64 {
        let a8 = b'a';
        let chars = s.as_bytes();
        let mut last = vec![Vec::new(); 26];
        let mut sum = 0;
        for (i, c) in chars.iter().enumerate() {
            let c = c - a8;
            if let Some(j) = last[(25 - c as usize) % 26].pop() {
                sum += (i - j) as i64;
            } else {
                last[c as usize].push(i);
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
        let s: String = "aczzx".into();
        let res = Solution::calculate_score(s);
        let expected: i64 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "abcdef".into();
        let res = Solution::calculate_score(s);
        let expected: i64 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
