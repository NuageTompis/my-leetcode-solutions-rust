struct Solution;

impl Solution {
    pub fn min_number_operations(target: Vec<i32>) -> i32 {
        let mut res = target[0];
        for pair in target.windows(2) {
            if pair[1] - pair[0] > 0 {
                res += pair[1] - pair[0];
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
        let target: Vec<i32> = vec![1, 2, 3, 2, 1];
        let res = Solution::min_number_operations(target);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let target: Vec<i32> = vec![3, 1, 1, 2];
        let res = Solution::min_number_operations(target);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let target: Vec<i32> = vec![3, 1, 5, 4, 2];
        let res = Solution::min_number_operations(target);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let target: Vec<i32> = vec![4, 1, 3, 4, 1, 2, 1, 3];
        let res = Solution::min_number_operations(target);
        let expected: i32 = 10; // Fill in this value
        assert_eq!(res, expected);
    }
}
