struct Solution;

impl Solution {
    pub fn sum_indices_with_k_set_bits(nums: Vec<i32>, k: i32) -> i32 {
        let mut res = 0;
        let k = k as u32;
        for (i, x) in nums.iter().enumerate() {
            if i.count_ones() == k {
                res += x;
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
        let nums: Vec<i32> = vec![5, 10, 1, 5, 2];
        let k: i32 = 1;
        let res = Solution::sum_indices_with_k_set_bits(nums, k);
        let expected: i32 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![4, 3, 2, 1];
        let k: i32 = 2;
        let res = Solution::sum_indices_with_k_set_bits(nums, k);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
