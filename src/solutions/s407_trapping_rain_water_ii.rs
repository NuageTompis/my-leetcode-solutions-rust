struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    pub fn trap_rain_water(mut height_map: Vec<Vec<i32>>) -> i32 {
        let m = height_map.len();
        let n = height_map[0].len();

        let mut borders: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new(); // (height, i, j)
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            borders.push((Reverse(height_map[i][0]), i, 0));
            visited[i][0] = true;
            borders.push((Reverse(height_map[i][n - 1]), i, n - 1));
            visited[i][n - 1] = true;
        }
        for j in 1..n - 1 {
            borders.push((Reverse(height_map[0][j]), 0, j));
            visited[0][j] = true;
            borders.push((Reverse(height_map[m - 1][j]), m - 1, j));
            visited[m - 1][j] = true;
        }

        let mut total = 0;
        let mut amt = borders.len();

        while amt < m * n {
            let (h, i, j) = borders.pop().unwrap();
            let h = h.0;

            for (di, dj) in Self::DIRS {
                let (i2, j2) = (i as i32 + di, j as i32 + dj);
                if i2 < 0 || j2 < 0 || i2 >= m as i32 || j2 >= n as i32 {
                    continue;
                }
                let (i2, j2) = (i2 as usize, j2 as usize);
                if !visited[i2][j2] {
                    if height_map[i2][j2] < h {
                        total += h - height_map[i2][j2];
                        height_map[i2][j2] = h;
                    }
                    borders.push((Reverse(height_map[i2][j2]), i2, j2));
                    visited[i2][j2] = true;
                    amt += 1;
                }
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let height_map: Vec<Vec<i32>> = vec![
            vec![1, 4, 3, 1, 3, 2],
            vec![3, 2, 1, 3, 2, 4],
            vec![2, 3, 3, 2, 3, 1],
        ];
        let res = Solution::trap_rain_water(height_map);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let height_map: Vec<Vec<i32>> = vec![
            vec![3, 3, 3, 3, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 2, 1, 2, 3],
            vec![3, 2, 2, 2, 3],
            vec![3, 3, 3, 3, 3],
        ];
        let res = Solution::trap_rain_water(height_map);
        let expected: i32 = 10; // Fill in this value
        assert_eq!(res, expected);
    }
}
