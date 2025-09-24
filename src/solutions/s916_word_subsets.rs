struct Solution;

impl Solution {
    pub fn word_subsets(words1: Vec<String>, words2: Vec<String>) -> Vec<String> {
        let occur = Self::max_occur(words2);
        let amt = occur.iter().sum::<u32>() as usize;

        let a = b'a';
        words1
            .iter()
            .filter(|&w| {
                if w.len() < amt {
                    false
                } else {
                    let mut cpt = 0;
                    let mut amt_temp = occur.clone();
                    for c in w.as_bytes() {
                        if amt_temp[(c - a) as usize] != 0 {
                            amt_temp[(c - a) as usize] -= 1;
                            cpt += 1;
                            if cpt == amt {
                                return true;
                            }
                        }
                    }
                    false
                }
            })
            .cloned()
            .collect()
    }

    pub fn max_occur(words: Vec<String>) -> Vec<u32> {
        // (most occurences, current) for each letter
        let mut res = [(0, 0); 26];

        let a = b'a';
        for w in &words {
            for c in w.as_bytes() {
                res[(c - a) as usize].1 += 1;
            }

            for occ in &mut res {
                occ.0 = occ.0.max(occ.1);
                occ.1 = 0;
            }
        }

        res.iter().map(|tuple| tuple.0).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words1: Vec<String> = vec![
            "amazon".into(),
            "apple".into(),
            "facebook".into(),
            "google".into(),
            "leetcode".into(),
        ];
        let words2: Vec<String> = vec!["e".into(), "o".into()];
        let res = Solution::word_subsets(words1, words2);
        let expected: Vec<String> = vec!["facebook".into(), "google".into(), "leetcode".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words1: Vec<String> = vec![
            "amazon".into(),
            "apple".into(),
            "facebook".into(),
            "google".into(),
            "leetcode".into(),
        ];
        let words2: Vec<String> = vec!["lc".into(), "eo".into()];
        let res = Solution::word_subsets(words1, words2);
        let expected: Vec<String> = vec!["leetcode".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let words1: Vec<String> = vec![
            "acaac".into(),
            "cccbb".into(),
            "aacbb".into(),
            "caacc".into(),
            "bcbbb".into(),
        ];
        let words2: Vec<String> = vec!["c".into(), "cc".into(), "b".into()];
        let res = Solution::word_subsets(words1, words2);
        let expected: Vec<String> = vec!["cccbb".into()]; // Fill in this value
        assert_eq!(res, expected);
    }
}
