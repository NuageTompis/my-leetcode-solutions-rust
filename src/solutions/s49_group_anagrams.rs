use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<Vec<u8>, Vec<usize>> = HashMap::new();

        for (i, s) in strs.iter().enumerate() {
            let mut s_bytes = s.as_bytes().to_vec();
            s_bytes.sort();
            map.entry(s_bytes)
                .and_modify(|v| v.push(i))
                .or_insert(vec![i]);
        }

        let mut res = Vec::new();
        for (_, v) in map {
            let mut group = Vec::new();
            for i in v {
                group.push(strs[i].clone());
            }
            res.push(group);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let strs: Vec<String> = vec![
            "eat".into(),
            "tea".into(),
            "tan".into(),
            "ate".into(),
            "nat".into(),
            "bat".into(),
        ];
        let mut res = Solution::group_anagrams(strs);
        let mut expected: Vec<Vec<String>> = vec![
            vec!["bat".into()],
            vec!["nat".into(), "tan".into()],
            vec!["ate".into(), "eat".into(), "tea".into()],
        ]; // Fill in this value
        res.iter_mut().for_each(|vec| vec.sort());
        res.sort();
        expected.iter_mut().for_each(|vec| vec.sort());
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let strs: Vec<String> = vec!["".into()];
        let mut res = Solution::group_anagrams(strs);
        let mut expected: Vec<Vec<String>> = vec![vec!["".into()]]; // Fill in this value
        res.iter_mut().for_each(|vec| vec.sort());
        res.sort();
        expected.iter_mut().for_each(|vec| vec.sort());
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let strs: Vec<String> = vec!["a".into()];
        let mut res = Solution::group_anagrams(strs);
        let mut expected: Vec<Vec<String>> = vec![vec!["a".into()]]; // Fill in this value
        res.iter_mut().for_each(|vec| vec.sort());
        res.sort();
        expected.iter_mut().for_each(|vec| vec.sort());
        expected.sort();
        assert_eq!(res, expected);
    }
}
