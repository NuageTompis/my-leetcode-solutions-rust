struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut sum = 0;
        for (i, w) in words.iter().enumerate() {
            for w2 in words.iter().skip(i + 1) {
                if w2.len() >= w.len() && w2.starts_with(w) && w2.ends_with(w) {
                    sum += 1;
                }
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
        let words: Vec<String> = vec!["a".into(), "aba".into(), "ababa".into(), "aa".into()];
        let res = Solution::count_prefix_suffix_pairs(words);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["pa".into(), "papa".into(), "ma".into(), "mama".into()];
        let res = Solution::count_prefix_suffix_pairs(words);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let words: Vec<String> = vec!["abab".into(), "ab".into()];
        let res = Solution::count_prefix_suffix_pairs(words);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
