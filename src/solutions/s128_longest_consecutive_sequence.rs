use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut seen = HashSet::new();
        let mut highs: HashMap<i32, i32> = HashMap::new();
        let mut lows: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            if seen.contains(n) {
                continue;
            }
            seen.insert(*n);
            if let Some(low) = highs.get(&(n - 1)) {
                let low = *low;

                if let Some(high) = lows.remove(&(n + 1)) {
                    highs.remove(&(n - 1));
                    lows.insert(low, high);
                    highs.insert(high, low);
                } else {
                    highs.remove(&(n - 1));
                    highs.insert(*n, low);
                    lows.insert(low, *n);
                }
            } else if let Some(high) = lows.remove(&(n + 1)) {
                lows.insert(*n, high);
                highs.insert(high, *n);
            } else {
                lows.insert(*n, *n);
                highs.insert(*n, *n);
            }
        }

        let mut best = 0;
        for (low, high) in lows.iter() {
            best = best.max(high - low + 1);
        }
        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![100, 4, 200, 1, 3, 2];
        let res = Solution::longest_consecutive(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let res = Solution::longest_consecutive(nums);
        let expected: i32 = 9; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![1, 0, 1, 2];
        let res = Solution::longest_consecutive(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
