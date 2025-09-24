#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let one_u8 = '1' as u8;
        let board: Vec<Vec<u8>> = board
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| if *x == '.' { 255 } else { *x as u8 - one_u8 })
                    .collect::<Vec<u8>>()
            })
            .collect();

        let mut seen;
        for i in 0..9 {
            seen = vec![false; 9];
            // rows
            for j in 0..9 {
                let ndx = board[i][j] as usize;
                if ndx == 255 {
                    continue;
                }
                if seen[ndx] {
                    return false;
                }
                seen[ndx] = true;
            }
            seen = vec![false; 9];
            // colums
            for j in 0..9 {
                let ndx = board[j][i] as usize;
                if ndx == 255 {
                    continue;
                }
                if seen[ndx] {
                    return false;
                }
                seen[ndx] = true;
            }
            // sub-boxes
            seen = vec![false; 9];
            let box_ndx = (i / 3 * 3, i % 3 * 3);
            for j in 0..9 {
                let ndx = board[box_ndx.0 + j / 3][box_ndx.1 + j % 3] as usize;
                if ndx == 255 {
                    continue;
                }
                if seen[ndx] {
                    return false;
                }
                seen[ndx] = true;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = Solution::is_valid_sudoku(board);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let board: Vec<Vec<char>> = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = Solution::is_valid_sudoku(board);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
