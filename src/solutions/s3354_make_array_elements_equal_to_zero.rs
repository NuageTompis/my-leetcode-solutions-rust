struct Solution;

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        // left prefix sum
        let mut left = Vec::with_capacity(n);
        nums.iter().fold(0, |acc, &x| {
            left.push(acc);
            acc + x
        });
        // right prefix sum
        let mut right = Vec::with_capacity(n);
        nums.iter().rev().fold(0, |acc, &x| {
            right.push(acc);
            acc + x
        });
        right.reverse();

        let mut res = 0;
        for (&x, (&l, r)) in nums.iter().zip(left.iter().zip(right)) {
            if x == 0 {
                match (l - r).abs() {
                    0 => res += 2,
                    1 => res += 1,
                    _ => (),
                }
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
        let nums: Vec<i32> = vec![1, 0, 2, 0, 3];
        let res = Solution::count_valid_selections(nums);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![2, 3, 4, 0, 4, 1, 0];
        let res = Solution::count_valid_selections(nums);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![0, 0, 0];
        let res = Solution::count_valid_selections(nums);
        let expected: i32 = 6; // Fill in this value
        assert_eq!(res, expected);
    }
}
