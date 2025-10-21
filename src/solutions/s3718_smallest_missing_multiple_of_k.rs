use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

struct Solution;

impl Solution {
    pub fn missing_multiple(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for n in nums {
            if n % k == 0 {
                heap.push(Reverse(n));
            }
        }

        let mut minimum = k;
        while let Some(m) = heap.pop() {
            match m.0.cmp(&minimum) {
                Ordering::Less => {
                    continue;
                }
                Ordering::Equal => {
                    minimum += k;
                }
                Ordering::Greater => {
                    return minimum;
                }
            }
        }
        minimum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![8, 2, 3, 4, 6];
        let k: i32 = 2;
        let res = Solution::missing_multiple(nums, k);
        let expected: i32 = 10; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 4, 7, 10, 15];
        let k: i32 = 5;
        let res = Solution::missing_multiple(nums, k);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }
}
