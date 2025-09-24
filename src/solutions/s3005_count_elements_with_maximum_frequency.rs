use std::{cmp::Ordering, collections::HashMap};

struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut max_freq = 0;
        let mut sum = 0;
        let mut frequencies: HashMap<i32, i32> = HashMap::new();

        for x in nums {
            let freq = frequencies.entry(x).or_default();
            *freq += 1;
            match (*freq).cmp(&max_freq) {
                Ordering::Greater => {
                    max_freq = *freq;
                    sum = max_freq;
                }
                Ordering::Equal => {
                    sum += max_freq;
                }
                Ordering::Less => (),
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 2, 2, 3, 1, 4];
        let res = Solution::max_frequency_elements(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let res = Solution::max_frequency_elements(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![2, 2, 3, 3, 3];
        let res = Solution::max_frequency_elements(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
