use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        nums.retain(|&x| x > 0);

        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut res = 0;
        for (i, x) in nums.iter().take(n - 2).enumerate() {
            let mut j = i + 1;
            let mut k = j + 1;
            while j < n - 1 {
                let limit = x + nums[j];
                match limit.cmp(&nums[k]) {
                    Ordering::Less | Ordering::Equal => {
                        res += k - 1 - j;
                        j += 1;
                    }
                    Ordering::Greater => {
                        if k == n - 1 {
                            res += k - j;
                            j += 1;
                        } else {
                            k += 1;
                        }
                    }
                }
            }
        }

        res as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![2, 2, 3, 4];
        let res = Solution::triangle_number(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![4, 2, 3, 4];
        let res = Solution::triangle_number(nums);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![1, 1, 2, 3];
        let res = Solution::triangle_number(nums);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let nums: Vec<i32> = vec![2, 99, 99, 2, 50, 98, 97];
        let res = Solution::triangle_number(nums);
        let expected: i32 = 18; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let nums: Vec<i32> = vec![7, 0, 0, 0];
        let res = Solution::triangle_number(nums);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
