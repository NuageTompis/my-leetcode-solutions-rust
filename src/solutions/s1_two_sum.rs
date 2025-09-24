struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut table = std::collections::HashSet::new();
        for (i, x) in nums.iter().enumerate() {
            let y = target - x;
            if table.contains(&y) {
                for (j, z) in nums.iter().enumerate() {
                    if *z == y {
                        return vec![i as i32, j as i32];
                    }
                }
            }
            table.insert(x);
        }
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let res = Solution::two_sum(nums, target);
        let expected: Vec<i32> = vec![0, 1]; // Fill in this value
        let res: HashSet<_> = res.into_iter().collect();
        let expected = expected.into_iter().collect();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![3, 2, 4];
        let target: i32 = 6;
        let res = Solution::two_sum(nums, target);
        let expected: Vec<i32> = vec![1, 2]; // Fill in this value
        let res: HashSet<_> = res.into_iter().collect();
        let expected = expected.into_iter().collect();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![3, 3];
        let target: i32 = 6;
        let res = Solution::two_sum(nums, target);
        let expected: Vec<i32> = vec![0, 1]; // Fill in this value
        let res: HashSet<_> = res.into_iter().collect();
        let expected = expected.into_iter().collect();
        assert_eq!(res, expected);
    }
}
