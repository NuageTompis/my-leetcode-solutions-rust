struct Solution;

impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let half = n >> 1;
        let mut res = (1..=half).chain(-half..=-1).collect::<Vec<i32>>();
        if n % 2 != 0 {
            res.push(0);
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 5;
        let mut res = Solution::sum_zero(n);
        let mut expected: Vec<i32> = vec![-2, -1, 1, 2, 0]; // Fill in this value
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 3;
        let mut res = Solution::sum_zero(n);
        let mut expected: Vec<i32> = vec![-1, 0, 1]; // Fill in this value
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let n: i32 = 1;
        let mut res = Solution::sum_zero(n);
        let mut expected: Vec<i32> = vec![0]; // Fill in this value
        res.sort();
        expected.sort();
        assert_eq!(res, expected);
    }
}
