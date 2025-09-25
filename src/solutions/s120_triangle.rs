struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut minimal = Vec::with_capacity(triangle.len());
        minimal.push(triangle[0][0]);

        for i in 1..triangle.len() {
            minimal.push(minimal[i - 1] + triangle[i][i]);
            for j in (1..i).rev() {
                minimal[j] = triangle[i][j] + minimal[j].min(minimal[j - 1]);
            }
            minimal[0] += triangle[i][0];
        }

        *minimal.iter().min().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let triangle: Vec<Vec<i32>> = vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]];
        let res = Solution::minimum_total(triangle);
        let expected: i32 = 11; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let triangle: Vec<Vec<i32>> = vec![vec![-10]];
        let res = Solution::minimum_total(triangle);
        let expected: i32 = -10; // Fill in this value
        assert_eq!(res, expected);
    }
}
