#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();

        // descending
        for i in 0..n - 1 {
            let mut temp: Vec<i32> = (0..(n - i)).map(|k| grid[i + k][k]).collect();
            temp.sort_unstable_by(|a, b| b.cmp(a));
            for (k, x) in temp.iter().enumerate() {
                grid[i + k][k] = *x;
            }
        }

        // ascending
        for i in 0..n - 1 {
            let mut temp: Vec<i32> = (0..(n - i - 1)).map(|k| grid[k][i + k + 1]).collect();
            temp.sort_unstable();
            for (k, x) in temp.iter().enumerate() {
                grid[k][i + k + 1] = *x;
            }
        }

        grid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 7, 3], vec![9, 8, 2], vec![4, 5, 6]];
        let res = Solution::sort_matrix(grid);
        let expected: Vec<Vec<i32>> = vec![vec![8, 2, 3], vec![9, 6, 7], vec![4, 5, 1]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 2]];
        let res = Solution::sort_matrix(grid);
        let expected: Vec<Vec<i32>> = vec![vec![2, 1], vec![1, 0]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let grid: Vec<Vec<i32>> = vec![vec![1]];
        let res = Solution::sort_matrix(grid);
        let expected: Vec<Vec<i32>> = vec![vec![1]]; // Fill in this value
        assert_eq!(res, expected);
    }
}
