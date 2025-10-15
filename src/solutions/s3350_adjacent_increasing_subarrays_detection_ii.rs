struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut prev_arr_length = 0;
        let mut curr_arr_length = 1;
        for pair in nums.windows(2) {
            if pair[0] < pair[1] {
                curr_arr_length += 1;
                let k = prev_arr_length.min(curr_arr_length);
                res = res.max(k);
                res = res.max(curr_arr_length >> 1);
            } else {
                prev_arr_length = curr_arr_length;
                curr_arr_length = 1;
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
        let nums: Vec<i32> = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let res = Solution::max_increasing_subarrays(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let res = Solution::max_increasing_subarrays(nums);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }
}
