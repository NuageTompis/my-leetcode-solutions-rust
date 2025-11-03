struct Solution;

impl Solution {
    pub fn find_missing_elements(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();
        let mut res = Vec::new();
        let mut curr = nums[0];
        for x in nums {
            for y in curr..x {
                res.push(y);
            }
            curr = x + 1
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 4, 2, 5];
        let res = Solution::find_missing_elements(nums);
        let expected: Vec<i32> = vec![3]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![7, 8, 6, 9];
        let res = Solution::find_missing_elements(nums);
        let expected: Vec<i32> = vec![]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![5, 1];
        let res = Solution::find_missing_elements(nums);
        let expected: Vec<i32> = vec![2, 3, 4]; // Fill in this value
        assert_eq!(res, expected);
    }
}
