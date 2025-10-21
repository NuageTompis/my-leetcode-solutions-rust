struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        let mut avail = Solution::get_avail(&s);
        // try re-arranging s into target. add char one by one and also keep track of the minimum char greater available if exists
        // once we can't add same char, try minimum greater:
        // if exists add remaining in ascending order
        // else backtrack to previous existing

        let mut last_greater = None; // (index in target, character greater)
        for (i, c) in target.as_bytes().iter().enumerate() {
            let ndx = (c - b'a') as usize;
            let next_greater = avail[ndx + 1..]
                .iter()
                .enumerate()
                .find(|&(_, amt)| *amt > 0)
                .map(|(k, _)| k);
            if let Some(k) = next_greater {
                last_greater = Some((i, k + ndx + 1));
            }
            if avail[ndx] > 0 {
                avail[ndx] -= 1;
            } else {
                break;
            }
        }

        if let Some((i, c)) = last_greater {
            let mut res: Vec<u8> = target.as_bytes()[..i].into();
            res.push(c as u8 + b'a');
            let mut avail = Solution::get_avail(&s);
            for used in &res {
                avail[(used - b'a') as usize] -= 1;
            }
            for (i, &amt) in avail.iter().enumerate() {
                for _ in 0..amt {
                    res.push(i as u8 + b'a');
                }
            }
            String::from_utf8(res).unwrap()
        } else {
            String::new()
        }
    }

    fn get_avail(s: &str) -> [u32; 26] {
        let mut avail = [0; 26];
        for c in s.as_bytes() {
            avail[(c - b'a') as usize] += 1;
        }
        avail
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "abc".into();
        let target: String = "bba".into();
        let res = Solution::lex_greater_permutation(s, target);
        let expected: String = "bca".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "leet".into();
        let target: String = "code".into();
        let res = Solution::lex_greater_permutation(s, target);
        let expected: String = "eelt".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "baba".into();
        let target: String = "bbaa".into();
        let res = Solution::lex_greater_permutation(s, target);
        let expected: String = "".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
