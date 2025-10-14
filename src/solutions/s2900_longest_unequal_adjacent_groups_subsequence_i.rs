struct Solution;

impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut res = Vec::new();
        // previous: word, b (0 or 1)
        let mut prev_best = words[0].clone();
        let mut prev_b = groups[0];
        for (w, &b) in words.iter().zip(groups.iter()).skip(1) {
            match b {
                // same group
                x if x == prev_b => {
                    if w.len() > prev_best.len() {
                        prev_best = w.clone();
                    }
                }
                // new group
                x => {
                    prev_b = x;
                    res.push(prev_best.to_owned());
                }
            }
        }
        // add ending group
        res.push(prev_best);
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words: Vec<String> = vec!["c".into()];
        let groups: Vec<i32> = vec![0];
        let res = Solution::get_longest_subsequence(words, groups);
        let expected: Vec<String> = vec!["e".into(), "b".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["d".into()];
        let groups: Vec<i32> = vec![1];
        let res = Solution::get_longest_subsequence(words, groups);
        let expected: Vec<String> = vec!["a".into(), "b".into(), "c".into()]; // Fill in this value
        assert_eq!(res, expected);
    }
}
