use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut res = Vec::with_capacity(2);
        let mut seen = HashSet::with_capacity(nums.len() - 2);
        for x in nums {
            if seen.contains(&x) {
                res.push(x);
            } else {
                seen.insert(x);
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
        let nums: Vec<i32> = vec![0, 1, 1, 0];
        let res = Solution::get_sneaky_numbers(nums);
        let expected: Vec<i32> = vec![0, 1]; // Fill in this value
        assert_eq!(HashSet::<i32>::from_iter(res), HashSet::from_iter(expected));
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![0, 3, 2, 1, 3, 2];
        let res = Solution::get_sneaky_numbers(nums);
        let expected: Vec<i32> = vec![2, 3]; // Fill in this value
        assert_eq!(HashSet::<i32>::from_iter(res), HashSet::from_iter(expected));
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![7, 1, 5, 4, 3, 4, 6, 0, 9, 5, 8, 2];
        let res = Solution::get_sneaky_numbers(nums);
        let expected: Vec<i32> = vec![4, 5]; // Fill in this value
        assert_eq!(HashSet::<i32>::from_iter(res), HashSet::from_iter(expected));
    }
}
