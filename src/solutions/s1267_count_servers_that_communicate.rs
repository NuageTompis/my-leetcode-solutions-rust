struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut rows = vec![0; m];
        let mut cols = vec![0; n];

        for i in 0..m {
            #[allow(clippy::needless_range_loop)]
            for j in 0..n {
                if grid[i][j] == 1 {
                    rows[i] += 1;
                    cols[j] += 1;
                }
            }
        }

        let mut res = rows
            .iter()
            .filter_map(|&cpt| if cpt < 2 { None } else { Some(cpt) })
            .collect::<Vec<i32>>()
            .iter()
            .sum();

        for i in 0..m {
            #[allow(clippy::needless_range_loop)]
            for j in 0..n {
                if grid[i][j] == 1 && cols[j] >= 2 && rows[i] == 1 {
                    res += 1;
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
        let grid: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 1]];
        let res = Solution::count_servers(grid);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 0], vec![1, 1]];
        let res = Solution::count_servers(grid);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let grid: Vec<Vec<i32>> = vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1],
        ];
        let res = Solution::count_servers(grid);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }
}
