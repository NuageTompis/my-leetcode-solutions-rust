#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut sub = vec![nums[0]];
        for i in 1..n {
            if nums[i] > *sub.last().unwrap() {
                sub.push(nums[i]);
            } else {
                let ndx = match sub.binary_search(&nums[i]) {
                    Ok(x) => x,
                    Err(x) => x,
                };
                sub[ndx] = nums[i];
            }
        }

        sub.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![10, 9, 2, 5, 3, 7, 101, 18];
        let res = Solution::length_of_lis(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![0, 1, 0, 3, 2, 3];
        let res = Solution::length_of_lis(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![7, 7, 7, 7, 7, 7, 7];
        let res = Solution::length_of_lis(nums);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
