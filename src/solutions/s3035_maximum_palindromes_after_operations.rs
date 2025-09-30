struct Solution;

impl Solution {
    pub fn max_palindromes_after_operations(mut words: Vec<String>) -> i32 {
        let a = b'a';
        // maps a letter to its total amount modulo 2
        let mut odd_letters = [0; 26];
        // amount of words with odd length
        let mut odd_length_words = 0;
        for w in &words {
            if !w.len().is_multiple_of(2) {
                odd_length_words += 1;
            }
            for c in w.chars() {
                let ndx = (c as u8 - a) as usize;
                odd_letters[ndx] += 1;
            }
        }
        let odd_letters = odd_letters.map(|cpt| cpt % 2);
        let odd_letters_amt = odd_letters.iter().sum();

        // start by considering all words as palindromable
        let mut res = words.len() as i32;

        if odd_letters_amt <= odd_length_words {
            return res;
        }

        words.sort_unstable_by_key(|w2| std::cmp::Reverse(w2.len()));
        let mut words = words.iter();

        // n - res cannot be over 26 (but no need to check)
        let mut k = odd_length_words;
        while k < odd_letters_amt {
            let w = words.next().unwrap();
            if w.len().is_multiple_of(2) {
                k += w.len()
            } else {
                k += w.len() - 1
            };

            res -= 1; // abandon a word
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let words: Vec<String> = vec!["abbb".into(), "ba".into(), "aa".into()];
        let res = Solution::max_palindromes_after_operations(words);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let words: Vec<String> = vec!["abc".into(), "ab".into()];
        let res = Solution::max_palindromes_after_operations(words);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let words: Vec<String> = vec!["cd".into(), "ef".into(), "a".into()];
        let res = Solution::max_palindromes_after_operations(words);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
