struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        let mut res = 0;
        operations.iter().for_each(|op| {
            if op.as_bytes()[1] == b'+' {
                res += 1;
            } else {
                res -= 1;
            }
        });

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let operations: Vec<String> = vec!["--X".into(), "X++".into(), "X++".into()];
        let res = Solution::final_value_after_operations(operations);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let operations: Vec<String> = vec!["++X".into(), "++X".into(), "X++".into()];
        let res = Solution::final_value_after_operations(operations);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let operations: Vec<String> = vec!["X++".into(), "++X".into(), "--X".into(), "X--".into()];
        let res = Solution::final_value_after_operations(operations);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
