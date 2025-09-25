struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        let mut b = 0;
        let n = derived.len();
        for d in derived.iter().take(n - 1) {
            b ^= d;
        }
        derived[n - 1] == b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let derived: Vec<i32> = vec![1, 1, 0];
        let res = Solution::does_valid_array_exist(derived);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let derived: Vec<i32> = vec![1, 1];
        let res = Solution::does_valid_array_exist(derived);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let derived: Vec<i32> = vec![1, 0];
        let res = Solution::does_valid_array_exist(derived);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
