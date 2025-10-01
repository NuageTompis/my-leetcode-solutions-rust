use std::panic;

struct Solution;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut min_after = vec![0; n];
        let mut min = i32::MAX;
        for (i, &x) in nums.iter().rev().enumerate() {
            min = min.min(x);
            min_after[i] = min;
        }
        min_after.reverse();

        let mut max = 0;
        for (i, &x) in nums.iter().enumerate() {
            max = max.max(x);
            if max <= min_after[i + 1] {
                return i as i32 + 1;
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![5, 0, 3, 8, 6];
        let res = Solution::partition_disjoint(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 1, 1, 0, 6, 12];
        let res = Solution::partition_disjoint(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![1, 1, 1];
        let res = Solution::partition_disjoint(nums);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
