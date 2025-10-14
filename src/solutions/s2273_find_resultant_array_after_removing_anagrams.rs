struct Solution;

impl Solution {
    pub fn remove_anagrams(words: Vec<String>) -> Vec<String> {
        let mut prev_set = [0; 26];
        let mut prev_len = 0;
        let mut res = Vec::new();
        let mut prev_computed_ndx = None;

        for (i, w) in words.iter().enumerate() {
            if w.len() == prev_len {
                let should_compute_prev_set = match prev_computed_ndx {
                    None => true,
                    Some(ndx) => ndx != i - 1, // i cannot be zero
                };
                if should_compute_prev_set {
                    prev_set = Self::compute_set(&words[i - 1]);
                }
                // compute
                let mut differs_from_prev = false;
                let mut new_set = [0; 26];
                for c in w.as_bytes() {
                    let k = (c - b'a') as usize;
                    new_set[k] += 1;
                    if new_set[k] > prev_set[k] {
                        differs_from_prev = true;
                    }
                }
                if differs_from_prev {
                    res.push(w.clone());
                    prev_set = new_set;
                }
                prev_computed_ndx = Some(i);
            } else {
                res.push(w.clone());
                prev_len = w.len();
            }
        }

        res
    }

    fn compute_set(w: &str) -> [i32; 26] {
        let mut set = [0; 26];
        for c in w.as_bytes() {
            set[(c - b'a') as usize] += 1;
        }
        set
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words: Vec<String> = vec![
            "abba".into(),
            "baba".into(),
            "bbaa".into(),
            "cd".into(),
            "cd".into(),
        ];
        let res = Solution::remove_anagrams(words);
        let expected: Vec<String> = vec!["abba".into(), "cd".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()];
        let res = Solution::remove_anagrams(words);
        let expected: Vec<String> =
            vec!["a".into(), "b".into(), "c".into(), "d".into(), "e".into()]; // Fill in this value
        assert_eq!(res, expected);
    }
}
