use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>) -> i64 {
        let mut increasing = true;
        let mut equal = false;
        let mut ndx = 0;
        for (i, pair) in nums.windows(2).enumerate() {
            match pair[0].cmp(&pair[1]) {
                Ordering::Equal => {
                    if increasing {
                        equal = true;
                        increasing = false;
                        ndx = i;
                    } else {
                        return -1;
                    }
                }
                Ordering::Greater => {
                    if increasing {
                        increasing = false;
                        ndx = i;
                    }
                }
                Ordering::Less => {
                    if increasing {
                    } else {
                        return -1;
                    }
                }
            }
        }

        if equal {
            let left = nums[..=ndx].iter().fold(0, |acc, &x| acc + x as i64);
            let right = nums[ndx + 1..].iter().fold(0, |acc, &x| acc + x as i64);
            (left - right).abs()
        } else {
            if increasing {
                ndx = nums.len() - 1;
            }
            let left = nums[..ndx].iter().fold(0, |acc, &x| acc + x as i64);
            let right = nums[ndx + 1..].iter().fold(0, |acc, &x| acc + x as i64);
            (left + nums[ndx] as i64 - right)
                .abs()
                .min((right + nums[ndx] as i64 - left).abs())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 3, 2];
        let res = Solution::split_array(nums);
        let expected: i64 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 4, 3];
        let res = Solution::split_array(nums);
        let expected: i64 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![3, 1, 2];
        let res = Solution::split_array(nums);
        let expected: i64 = -1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let nums: Vec<i32> = vec![2, 3, 4, 5];
        let res = Solution::split_array(nums);
        let expected: i64 = 4; // Fill in this value
        assert_eq!(res, expected);
    }
}
