struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        // idea: build the grid by first placing the walls
        // then place the guards one by one, for each guard build his Line Of Sight (4 directions)
        // break early if the vertical/horizontal LOS is already marked
        // answer by counting the cells that remain unwatched at the end

        let n = n as usize;
        let m = m as usize;
        let mut grid = vec![vec![0; n]; m]; // 0 -> no LOS; 1,2 -> vertical, horizontal LOS; 4 -> wall

        // place walls
        for cell in walls {
            grid[cell[0] as usize][cell[1] as usize] = 4;
        }

        // mark guarded cells
        for cell in guards {
            let i = cell[0];
            let j = cell[1];
            grid[i as usize][j as usize] = 3;

            // conditions to keep going
            let vertical_condition = |i2: i32, grid: &Vec<Vec<i32>>| {
                // in bounds, no wall, no vertical LOS already marked
                i2 >= 0
                    && i2 < m as i32
                    && grid[i2 as usize][j as usize] != 4
                    && (grid[i2 as usize][j as usize] & 1 == 0)
            };
            let horizontal_condition = |j2: i32, grid: &Vec<Vec<i32>>| {
                // in bounds, no wall, no horizontal LOS already marked
                j2 >= 0
                    && j2 < n as i32
                    && grid[i as usize][j2 as usize] != 4
                    && (grid[i as usize][j2 as usize] & 2 == 0)
            };
            // up
            let mut i2 = i - 1;
            while vertical_condition(i2, &grid) {
                grid[i2 as usize][j as usize] |= 1;
                i2 -= 1;
            }
            // down
            let mut i2 = i + 1;
            while vertical_condition(i2, &grid) {
                grid[i2 as usize][j as usize] |= 1;
                i2 += 1;
            }
            // left
            let mut j2 = j - 1;
            while horizontal_condition(j2, &grid) {
                grid[i as usize][j2 as usize] |= 2;
                j2 -= 1;
            }
            // right
            let mut j2 = j + 1;
            while horizontal_condition(j2, &grid) {
                grid[i as usize][j2 as usize] |= 2;
                j2 += 1;
            }
        }

        // count how many cells are zeros (ie not guarded)
        grid.iter()
            .map(|row| row.iter().filter(|&&x| x == 0).count())
            .sum::<usize>() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let m: i32 = 4;
        let n: i32 = 6;
        let guards: Vec<Vec<i32>> = vec![vec![0, 0], vec![1, 1], vec![2, 3]];
        let walls: Vec<Vec<i32>> = vec![vec![0, 1], vec![2, 2], vec![1, 4]];
        let res = Solution::count_unguarded(m, n, guards, walls);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let m: i32 = 3;
        let n: i32 = 3;
        let guards: Vec<Vec<i32>> = vec![vec![1, 1]];
        let walls: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]];
        let res = Solution::count_unguarded(m, n, guards, walls);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }
}
