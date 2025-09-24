struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut dirs = Vec::new();

        let mut curr = String::new();
        for c in path.chars() {
            match c {
                '.' => {
                    curr.push('.');
                }
                '/' => {
                    if curr == ".." {
                        dirs.pop();
                    } else if curr == "." || curr.is_empty() {
                    } else {
                        dirs.push(curr);
                    }
                    curr = String::new();
                }
                _ => {
                    curr.push(c);
                }
            }
        }

        if !curr.is_empty() && curr != "." {
            if curr == ".." {
                dirs.pop();
            } else {
                dirs.push(curr);
            }
        }

        if dirs.is_empty() {
            return String::from("/");
        }

        let mut output = String::new();
        for d in dirs {
            output += "/";
            output += &d;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let path: String = "/home/".into();
        let res = Solution::simplify_path(path);
        let expected: String = "/home".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let path: String = "/home//foo/".into();
        let res = Solution::simplify_path(path);
        let expected: String = "/home/foo".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let path: String = "/home/user/Documents/../Pictures".into();
        let res = Solution::simplify_path(path);
        let expected: String = "/home/user/Pictures".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let path: String = "/../".into();
        let res = Solution::simplify_path(path);
        let expected: String = "/".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let path: String = "/.../a/../b/c/../d/./".into();
        let res = Solution::simplify_path(path);
        let expected: String = "/.../b/d".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
