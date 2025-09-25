struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut [Vec<i32>]) {
        let n = matrix.len();

        for i in 0..(n >> 1) {
            for j in i..n - 1 - i {
                let temp = matrix[i][j];
                let mut ndx = (i, j);
                for _ in 0..3 {
                    let next_ndx = nxt_ndx(ndx, n);
                    matrix[ndx.0][ndx.1] = matrix[next_ndx.0][next_ndx.1];
                    ndx = next_ndx;
                }
                matrix[ndx.0][ndx.1] = temp;
            }
        }
    }
}

pub fn nxt_ndx(ndx: (usize, usize), n: usize) -> (usize, usize) {
    (n - 1 - ndx.1, ndx.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut matrix: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        Solution::rotate(&mut matrix);
        let expected = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]; // Fill in this value
        assert_eq!(matrix, expected);
    }

    #[test]
    fn example_2() {
        let mut matrix: Vec<Vec<i32>> = vec![
            vec![5, 1, 9, 11],
            vec![2, 4, 8, 10],
            vec![13, 3, 6, 7],
            vec![15, 14, 12, 16],
        ];
        Solution::rotate(&mut matrix);
        let expected = vec![
            vec![15, 13, 2, 5],
            vec![14, 3, 4, 1],
            vec![12, 6, 8, 9],
            vec![16, 7, 10, 11],
        ]; // Fill in this value
        assert_eq!(matrix, expected);
    }
}
