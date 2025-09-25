struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut dico = std::collections::HashMap::new();

        for c in magazine.chars() {
            dico.entry(c).and_modify(|val| *val += 1).or_insert(1);
        }

        for c in ransom_note.chars() {
            match dico.get_mut(&c) {
                Some(val) if *val > 0 => *val -= 1,
                _ => return false,
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let ransom_note: String = "a".into();
        let magazine: String = "b".into();
        let res = Solution::can_construct(ransom_note, magazine);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let ransom_note: String = "aa".into();
        let magazine: String = "ab".into();
        let res = Solution::can_construct(ransom_note, magazine);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let ransom_note: String = "aa".into();
        let magazine: String = "aab".into();
        let res = Solution::can_construct(ransom_note, magazine);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
