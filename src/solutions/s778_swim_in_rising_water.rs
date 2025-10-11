use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

struct Solution;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut t;
        let mut unreachables = BinaryHeap::new();
        unreachables.push((Reverse(grid[0][0]), 0, 0));
        let mut visited = vec![vec![false; n]; n];
        loop {
            let (elevation, i, j) = unreachables.pop().unwrap();
            t = elevation.0 as usize;

            let mut recheables = VecDeque::from([(i, j)]);
            while let Some((i, j)) = recheables.pop_front() {
                if i == n - 1 && j == n - 1 {
                    return t as i32;
                }

                for (d, &(di, dj)) in Self::DIRS.iter().enumerate() {
                    let (i2, j2) = (i as i32 + di, j as i32 + dj);
                    if i2 < 0 || j2 < 0 || i2 == n as i32 || j2 == n as i32 {
                        continue;
                    }
                    let (i2, j2) = (i2 as usize, j2 as usize);

                    if visited[i2][j2] {
                        continue;
                    }
                    visited[i2][j2] = true;

                    if grid[i2][j2] as usize > t {
                        unreachables.push((Reverse(grid[i2][j2]), i2, j2));
                    } else if d.is_multiple_of(2) {
                        recheables.push_front((i2, j2));
                    } else {
                        recheables.push_back((i2, j2));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let grid: Vec<Vec<i32>> = vec![vec![0, 2], vec![1, 3]];
        let res = Solution::swim_in_water(grid);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let grid: Vec<Vec<i32>> = vec![
            vec![0, 1, 2, 3, 4],
            vec![24, 23, 22, 21, 5],
            vec![12, 13, 14, 15, 16],
            vec![11, 17, 18, 19, 20],
            vec![10, 9, 8, 7, 6],
        ];
        let res = Solution::swim_in_water(grid);
        let expected: i32 = 16; // Fill in this value
        assert_eq!(res, expected);
    }
}
