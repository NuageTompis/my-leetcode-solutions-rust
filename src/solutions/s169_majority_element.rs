struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut candidate = 0;

        for n in nums {
            if count == 0 {
                count = 1;
                candidate = n;
            } else if n == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![3, 2, 3];
        let res = Solution::majority_element(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![2, 2, 1, 1, 1, 2, 2];
        let res = Solution::majority_element(nums);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }
}
