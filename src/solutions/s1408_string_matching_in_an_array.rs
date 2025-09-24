struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        words
            .iter()
            .filter_map(|w| {
                for w2 in &words {
                    if w2.len() > w.len() && w2.contains(w) {
                        return Some(w.clone());
                    }
                }
                None
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words: Vec<String> = vec![
            "mass".into(),
            "as".into(),
            "hero".into(),
            "superhero".into(),
        ];
        let res = Solution::string_matching(words);
        let expected: Vec<String> = vec!["as".into(), "hero".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["leetcode".into(), "et".into(), "code".into()];
        let res = Solution::string_matching(words);
        let expected: Vec<String> = vec!["et".into(), "code".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let words: Vec<String> = vec!["blue".into(), "green".into(), "bu".into()];
        let res = Solution::string_matching(words);
        let expected: Vec<String> = vec![]; // Fill in this value
        assert_eq!(res, expected);
    }
}
