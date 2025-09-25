struct Solution;

impl Solution {
    pub fn game_of_life(board: &mut [Vec<i32>]) {
        // if alive
        // 1. 2 or 3 neighbors -> live
        // 2. else die
        // if dead
        // if 3 neighbors -> live

        let n = board.len();
        let m = board[0].len();
        for i in 0..n {
            for j in 0..m {
                let mut neighbors = 0;
                for k in 0..3 {
                    for l in 0..3 {
                        if (k == 1) & (l == 1) {
                            continue;
                        }
                        let i2 = i + k;
                        let j2 = j + l;
                        if (i2 == 0) || (j2 == 0) || (i2 == n + 1) || (j2 == m + 1) {
                            continue;
                        }
                        if board[i2 - 1][j2 - 1] & 1 == 1 {
                            neighbors += 1;
                        }
                    }
                }
                if board[i][j] & 1 == 1 {
                    if (neighbors == 2) || (neighbors == 3) {
                        board[i][j] = 3;
                    }
                } else if neighbors == 3 {
                    board[i][j] = 2;
                }
            }
        }
        #[allow(clippy::needless_range_loop)]
        for i in 0..n {
            for j in 0..m {
                board[i][j] >>= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut board: Vec<Vec<i32>> =
            vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
        Solution::game_of_life(&mut board);
        let expected = vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]; // Fill in this value
        assert_eq!(board, expected);
    }

    #[test]
    fn example_2() {
        let mut board: Vec<Vec<i32>> = vec![vec![1, 1], vec![1, 0]];
        Solution::game_of_life(&mut board);
        let expected = vec![vec![1, 1], vec![1, 1]]; // Fill in this value
        assert_eq!(board, expected);
    }
}
