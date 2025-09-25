struct Solution;

impl Solution {
    pub fn compressed_string(word: String) -> String {
        let mut res = String::new();
        let mut chars = word.chars();
        let mut last = chars.next().unwrap();
        let mut cpt: u8 = 1;

        for c in chars {
            if c == last && cpt != 9 {
                cpt += 1;
            } else {
                res.push((cpt + b'0') as char);
                res.push(last);
                cpt = 1;
                last = c;
            }
        }

        res.push((cpt + b'0') as char);
        res.push(last);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let word: String = "abcde".into();
        let res = Solution::compressed_string(word);
        let expected: String = "1a1b1c1d1e".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let word: String = "aaaaaaaaaaaaaabb".into();
        let res = Solution::compressed_string(word);
        let expected: String = "9a5a2b".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
