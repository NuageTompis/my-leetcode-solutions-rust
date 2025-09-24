use std::collections::VecDeque;

struct Solution;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut min_costs = vec![vec![i32::MAX; m]; n];
        min_costs[0][0] = 0;

        let mut queue = VecDeque::new();
        queue.push_front((0, 0, 0)); // (cost, i, j)

        while let Some((c, i, j)) = queue.pop_front() {
            for (d, &(di, dj)) in Self::DIRS.iter().enumerate() {
                let (i2, j2) = (i as i32 + di, j as i32 + dj);
                if i2 < 0 || j2 < 0 || i2 >= n as i32 || j2 >= m as i32 {
                    continue;
                }

                let (i2, j2) = (i2 as usize, j2 as usize);

                let new_c = if grid[i][j] == d as i32 + 1 { c } else { c + 1 };
                if new_c >= min_costs[i2][j2] {
                    continue;
                }

                min_costs[i2][j2] = new_c;
                if new_c > c {
                    queue.push_back((new_c, i2, j2));
                } else {
                    queue.push_front((new_c, i2, j2));
                }
            }
        }

        min_costs[n - 1][m - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid: Vec<Vec<i32>> = vec![
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
            vec![1, 1, 1, 1],
            vec![2, 2, 2, 2],
        ];
        let res = Solution::min_cost(grid);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 1, 3], vec![3, 2, 2], vec![1, 1, 4]];
        let res = Solution::min_cost(grid);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let grid: Vec<Vec<i32>> = vec![vec![1, 2], vec![4, 3]];
        let res = Solution::min_cost(grid);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
