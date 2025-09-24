use std::collections::VecDeque;

struct Solution;

impl Solution {
    const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    pub fn highest_peak(mut is_water: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::new();
        let m = is_water.len();
        let n = is_water[0].len();

        #[allow(clippy::needless_range_loop)]
        for i in 0..m {
            for j in 0..n {
                is_water[i][j] = if is_water[i][j] == 0 {
                    -1
                } else {
                    queue.push_back((i, j)); // (x, y, height)
                    0
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            for (di, dj) in Self::DIRS.iter() {
                let (i2, j2) = (i as i32 + di, j as i32 + dj);
                if i2 < 0 || j2 < 0 || i2 >= m as i32 || j2 >= n as i32 {
                    continue;
                }

                let (i2, j2) = (i2 as usize, j2 as usize);
                if is_water[i2][j2] == -1 {
                    is_water[i2][j2] = is_water[i][j] + 1;
                    queue.push_back((i2, j2));
                }
            }
        }

        is_water
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let is_water: Vec<Vec<i32>> = vec![vec![0, 1], vec![0, 0]];
        let res = Solution::highest_peak(is_water);
        let expected: Vec<Vec<i32>> = vec![vec![1, 0], vec![2, 1]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let is_water: Vec<Vec<i32>> = vec![vec![0, 0, 1], vec![1, 0, 0], vec![0, 0, 0]];
        let res = Solution::highest_peak(is_water);
        let expected: Vec<Vec<i32>> = vec![vec![1, 1, 0], vec![0, 1, 1], vec![1, 2, 2]]; // Fill in this value
        assert_eq!(res, expected);
    }
}
