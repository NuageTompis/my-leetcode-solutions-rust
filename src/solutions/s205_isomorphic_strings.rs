use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut dico_st = HashMap::new();
        let mut dico_ts = HashMap::new();
        let mut new_s = String::new();

        for (i, c) in s.chars().enumerate() {
            let c2 = match dico_st.get(&c) {
                Some(&c2) => c2,
                _ => {
                    let c2 = t.chars().nth(i).unwrap();
                    if dico_ts.contains_key(&c2) {
                        return false;
                    }
                    dico_ts.insert(c2, c);
                    dico_st.insert(c, c2);
                    c2
                }
            };
            new_s.push(c2);
        }

        new_s == t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "egg".into();
        let t: String = "add".into();
        let res = Solution::is_isomorphic(s, t);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "foo".into();
        let t: String = "bar".into();
        let res = Solution::is_isomorphic(s, t);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "paper".into();
        let t: String = "title".into();
        let res = Solution::is_isomorphic(s, t);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
