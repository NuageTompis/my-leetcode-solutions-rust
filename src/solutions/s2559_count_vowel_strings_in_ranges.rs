struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let vowels: Vec<bool> = words
            .iter()
            .map(|s| {
                vowels.contains(&s.chars().last().unwrap())
                    && vowels.contains(&s.chars().next().unwrap())
            })
            .collect();

        let mut prefix_sum = vec![0; words.len()];
        let mut curr = 0;
        for i in 0..words.len() {
            if vowels[i] {
                curr += 1;
            }
            prefix_sum[i] = curr;
        }

        let mut res = vec![0; queries.len()];

        for i in 0..queries.len() {
            res[i] = prefix_sum[queries[i][1] as usize];
            if queries[i][0] > 0 {
                res[i] -= prefix_sum[queries[i][0] as usize - 1];
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words: Vec<String> = vec![
            "aba".into(),
            "bcb".into(),
            "ece".into(),
            "aa".into(),
            "e".into(),
        ];
        let queries: Vec<Vec<i32>> = vec![vec![0, 2], vec![1, 4], vec![1, 1]];
        let res = Solution::vowel_strings(words, queries);
        let expected: Vec<i32> = vec![2, 3, 0]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["a".into(), "e".into(), "i".into()];
        let queries: Vec<Vec<i32>> = vec![vec![0, 2], vec![0, 1], vec![2, 2]];
        let res = Solution::vowel_strings(words, queries);
        let expected: Vec<i32> = vec![3, 2, 1]; // Fill in this value
        assert_eq!(res, expected);
    }
}
