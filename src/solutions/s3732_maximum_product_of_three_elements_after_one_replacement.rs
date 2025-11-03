struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i64 {
        let mut max = [i32::MIN; 3]; // max, second, third
        for x in nums {
            let x = x.abs();
            if x > max[0] {
                max[2] = max[1];
                max[1] = max[0];
                max[0] = x;
            } else if x > max[1] {
                max[2] = max[1];
                max[1] = x;
            } else {
                max[2] = max[2].max(x);
            }
        }

        max[0] as i64 * max[1] as i64 * 100_000
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![-5, 7, 0];
        let res = Solution::max_product(nums);
        let expected: i64 = 3500000; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![-4, -2, -1, -3];
        let res = Solution::max_product(nums);
        let expected: i64 = 1200000; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![0, 10, 0];
        let res = Solution::max_product(nums);
        let expected: i64 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
