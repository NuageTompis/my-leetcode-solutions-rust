struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut queue = std::collections::VecDeque::new();
        for c in s.chars() {
            let (ndx, open) = match c {
                '(' => (0, true),
                ')' => (0, false),
                '{' => (1, true),
                '}' => (1, false),
                '[' => (2, true),
                ']' => (2, false),
                _ => panic!("wrong input"),
            };
            if open {
                queue.push_back(ndx);
            } else {
                match queue.pop_back() {
                    None => return false,
                    Some(x) => {
                        if x != ndx {
                            return false;
                        }
                    }
                }
            }
        }

        queue.pop_back().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "()".into();
        let res = Solution::is_valid(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "()[]{}".into();
        let res = Solution::is_valid(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "(]".into();
        let res = Solution::is_valid(s);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let s: String = "([])".into();
        let res = Solution::is_valid(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let s: String = "([)]".into();
        let res = Solution::is_valid(s);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
