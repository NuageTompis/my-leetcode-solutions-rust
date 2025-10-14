use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        if k == 1 {
            return true;
        }
        let mut cpt = 1;
        // tells wether we had a recent decrease but before it is a valid subarray
        let mut need_only_k = false;
        for pair in nums.windows(2) {
            match pair[0].cmp(&pair[1]) {
                Ordering::Less => {
                    cpt += 1;
                    if (cpt == k << 1) || (need_only_k && cpt == k) {
                        return true;
                    }
                }
                _ => {
                    need_only_k = cpt >= k;
                    cpt = 1;
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let k: i32 = 3;
        let res = Solution::has_increasing_subarrays(nums, k);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let k: i32 = 5;
        let res = Solution::has_increasing_subarrays(nums, k);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![-15, 19];
        let k: i32 = 1;
        let res = Solution::has_increasing_subarrays(nums, k);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let nums: Vec<i32> = vec![5, 8, -2, -1];
        let k: i32 = 2;
        let res = Solution::has_increasing_subarrays(nums, k);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
