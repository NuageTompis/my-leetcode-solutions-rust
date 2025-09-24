struct Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut map = [0; 123];
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();

        for &c in t_bytes {
            map[c as usize] += 1;
        }

        let mut needed = t.len();
        let mut left = 0;
        let mut best = usize::MAX;
        let mut best_left = 0;

        for right in 0..s.len() {
            if map[s_bytes[right] as usize] > 0 {
                needed -= 1;
            }
            map[s_bytes[right] as usize] -= 1;

            // shrink window if we have all we need
            while needed == 0 {
                map[s_bytes[left] as usize] += 1;
                if map[s_bytes[left] as usize] > 0 {
                    needed += 1;
                    let curr = right - left + 1;
                    if curr < best {
                        best = curr;
                        best_left = left;
                    }
                }
                left += 1;
            }
        }

        if best == usize::MAX {
            String::new()
        } else {
            s[best_left..best_left + best].to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "ADOBECODEBANC".into();
        let t: String = "ABC".into();
        let res = Solution::min_window(s, t);
        let expected: String = "BANC".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "a".into();
        let t: String = "a".into();
        let res = Solution::min_window(s, t);
        let expected: String = "a".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "a".into();
        let t: String = "aa".into();
        let res = Solution::min_window(s, t);
        let expected: String = "".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
