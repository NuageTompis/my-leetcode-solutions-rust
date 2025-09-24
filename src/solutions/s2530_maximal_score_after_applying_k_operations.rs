struct Solution;

impl Solution {
    pub fn max_kelements(nums: Vec<i32>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut sum = 0;
        for x in nums {
            heap.push(x);
        }
        for _ in 0..k {
            let mut x = heap.pop().unwrap();
            sum += x as i64;
            x = (x + 2) / 3;
            heap.push(x);
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![10, 10, 10, 10, 10];
        let k: i32 = 5;
        let res = Solution::max_kelements(nums, k);
        let expected: i64 = 50; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 10, 3, 3, 3];
        let k: i32 = 3;
        let res = Solution::max_kelements(nums, k);
        let expected: i64 = 17; // Fill in this value
        assert_eq!(res, expected);
    }
}
