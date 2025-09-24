struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let a_u8 = b'a';
        let mut broken = [false; 26];

        let broken_letters = broken_letters.as_bytes();
        for l in broken_letters {
            let ndx = (l - a_u8) as usize;
            broken[ndx] = true;
        }

        let mut cpt = 0;
        let words = text.split_whitespace().map(|word| word.as_bytes());
        for w in words {
            let mut is_broken = false;
            for l in w {
                let ndx = (l - a_u8) as usize;
                if broken[ndx] {
                    is_broken = true;
                    break;
                }
            }
            if !is_broken {
                cpt += 1;
            }
        }

        cpt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let text: String = "hello world".into();
        let broken_letters: String = "ad".into();
        let res = Solution::can_be_typed_words(text, broken_letters);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let text: String = "leet code".into();
        let broken_letters: String = "lt".into();
        let res = Solution::can_be_typed_words(text, broken_letters);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let text: String = "leet code".into();
        let broken_letters: String = "e".into();
        let res = Solution::can_be_typed_words(text, broken_letters);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
