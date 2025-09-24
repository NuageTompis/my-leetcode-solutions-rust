struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut chars = sentence.chars();
        let first = chars.next().unwrap();
        let mut last = first;

        let mut new = false;
        for c in chars {
            match c {
                ' ' => {
                    new = true;
                }
                _ => {
                    if new && c != last {
                        return false;
                    }
                    last = c;
                    new = false;
                }
            }
        }

        last == first
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let sentence: String = "leetcode exercises sound delightful".into();
        let res = Solution::is_circular_sentence(sentence);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let sentence: String = "eetcode".into();
        let res = Solution::is_circular_sentence(sentence);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let sentence: String = "Leetcode is cool".into();
        let res = Solution::is_circular_sentence(sentence);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
