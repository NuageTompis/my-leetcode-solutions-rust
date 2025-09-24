use std::{cmp::Ordering, mem::swap};

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let mut version1: Vec<u32> = version1
            .split('.')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        let mut version2: Vec<u32> = version2
            .split('.')
            .map(|v| v.parse::<u32>().unwrap())
            .collect();
        for (v1, v2) in version1.iter().zip(version2.iter()) {
            match v1.cmp(v2) {
                Ordering::Less => return -1,
                Ordering::Greater => return 1,
                Ordering::Equal => (),
            }
        }

        // version 1 is shorter
        let v1_is_smallest = if version2.len() < version1.len() {
            swap(&mut version1, &mut version2);
            false
        } else {
            true
        };

        for v in version2.iter().skip(version1.len()) {
            if *v != 0 {
                return if v1_is_smallest { -1 } else { 1 };
            }
        }

        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let version1: String = "1.2".into();
        let version2: String = "1.10".into();
        let res = Solution::compare_version(version1, version2);
        let expected: i32 = -1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let version1: String = "1.01".into();
        let version2: String = "1.001".into();
        let res = Solution::compare_version(version1, version2);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let version1: String = "1.0".into();
        let version2: String = "1.0.0.0".into();
        let res = Solution::compare_version(version1, version2);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let version1: String = "1.0.1".into();
        let version2: String = "1".into();
        let res = Solution::compare_version(version1, version2);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
