struct Solution;

impl Solution {
    pub fn grid_game(grid: Vec<Vec<i32>>) -> i64 {
        let n = grid[0].len();
        let mut worst = i64::MAX;
        let mut left = vec![0; n];
        let mut right = vec![0; n];
        let mut sum = 0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            sum += grid[1][i] as i64;
            left[i] = sum;
        }
        sum = 0;
        for i in (0..n).rev() {
            sum += grid[0][i] as i64;
            right[i] = sum;
        }
        for i in 0..n {
            let score2 = (if i == 0 { 0 } else { left[i - 1] }).max(if i == n - 1 {
                0
            } else {
                right[i + 1]
            });
            worst = worst.min(score2);
        }
        worst
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid: Vec<Vec<i32>> = vec![vec![2, 5, 4], vec![1, 5, 1]];
        let res = Solution::grid_game(grid);
        let expected: i64 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<i32>> = vec![vec![3, 3, 1], vec![8, 5, 2]];
        let res = Solution::grid_game(grid);
        let expected: i64 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]];
        let res = Solution::grid_game(grid);
        let expected: i64 = 7; // Fill in this value
        assert_eq!(res, expected);
    }
}
