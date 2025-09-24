struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut lefts: Vec<usize> = Vec::new();
        lefts.push(0);
        for i in 1..nums.len() {
            if nums[i] < nums[lefts[lefts.len() - 1]] {
                lefts.push(i);
            }
        }

        let mut best = 0;

        for r in (0..nums.len()).rev() {
            while !lefts.is_empty() && nums[lefts[lefts.len() - 1]] <= nums[r] {
                best = best.max(r - lefts[lefts.len() - 1]);
                lefts.pop();
            }
        }
        best as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![6, 0, 8, 2, 1, 5];
        let res = Solution::max_width_ramp(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1];
        let res = Solution::max_width_ramp(nums);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
    }
}
