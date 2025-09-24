struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        let mut sum = 0;
        for w in &words {
            if w.starts_with(&pref) {
                sum += 1;
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
        let words: Vec<String> = vec![
            "pay".into(),
            "attention".into(),
            "practice".into(),
            "attend".into(),
        ];
        let pref: String = "at".into();
        let res = Solution::prefix_count(words, pref);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec![
            "leetcode".into(),
            "win".into(),
            "loops".into(),
            "success".into(),
        ];
        let pref: String = "code".into();
        let res = Solution::prefix_count(words, pref);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
