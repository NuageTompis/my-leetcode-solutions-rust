struct Solution;

impl Solution {
    pub fn ways_to_split_array(nums: Vec<i32>) -> i32 {
        let mut diff: i64 = nums.iter().fold(0, |acc, x| acc - *x as i64) + 2 * nums[0] as i64;

        let mut cpt = 0;
        for x in nums.iter().skip(1) {
            if diff >= 0 {
                cpt += 1;
            }
            diff += (*x as i64) << 1;
        }

        cpt
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![10, 4, -8, 7];
        let res = Solution::ways_to_split_array(nums);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![2, 3, 1, 0];
        let res = Solution::ways_to_split_array(nums);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }
}
