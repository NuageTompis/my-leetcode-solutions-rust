struct Solution;

impl Solution {
    pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
        let n = folder.len();
        let mut folder = folder.clone();
        folder.sort();
        let mut res = Vec::new();
        let mut ndx = 0;
        while ndx < n {
            res.push(folder[ndx].clone());
            let mut ndx2 = ndx + 1;
            let l = folder[ndx].len();
            while ndx2 < n {
                let is_subfolder = folder[ndx2].len() > l
                    && folder[ndx2][..l] == folder[ndx]
                    && folder[ndx2].chars().nth(l).unwrap() == '/';

                if is_subfolder {
                    ndx2 += 1;
                } else {
                    ndx = ndx2;
                    break;
                }
            }
            if ndx2 == n {
                break;
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
        let folder: Vec<String> = vec![
            "/a".into(),
            "/a/b".into(),
            "/c/d".into(),
            "/c/d/e".into(),
            "/c/f".into(),
        ];
        let res = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a".into(), "/c/d".into(), "/c/f".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let folder: Vec<String> = vec!["/a".into(), "/a/b/c".into(), "/a/b/d".into()];
        let res = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a".into()]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let folder: Vec<String> = vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()];
        let res = Solution::remove_subfolders(folder);
        let expected: Vec<String> = vec!["/a/b/c".into(), "/a/b/ca".into(), "/a/b/d".into()]; // Fill in this value
        assert_eq!(res, expected);
    }
}
