struct Solution;

impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.reverse();
        nums.windows(3)
            .find(|&tri| tri[2] + tri[1] > tri[0])
            .map(|tri| tri.iter().sum())
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![2, 1, 2];
        let res = Solution::largest_perimeter(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 1, 10];
        let res = Solution::largest_perimeter(nums);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
