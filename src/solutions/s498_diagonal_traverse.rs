#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let n = mat.len();
        let m = mat[0].len();
        let mut res = vec![0; n * m];
        let mut up = true;
        let (mut i, mut j) = (0, 0);
        for k in 0..n * m {
            res[k] = mat[i][j];
            if up {
                if j == m - 1 {
                    up = false;
                    i += 1;
                } else if i == 0 {
                    up = false;
                    j += 1;
                } else {
                    i -= 1;
                    j += 1;
                }
            } else {
                if i == n - 1 {
                    up = true;
                    j += 1;
                } else if j == 0 {
                    up = true;
                    i += 1;
                } else {
                    i += 1;
                    j -= 1;
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
        let mat: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let res = Solution::find_diagonal_order(mat);
        let expected: Vec<i32> = vec![1, 2, 4, 7, 5, 3, 6, 8, 9]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let mat: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4]];
        let res = Solution::find_diagonal_order(mat);
        let expected: Vec<i32> = vec![1, 2, 3, 4]; // Fill in this value
        assert_eq!(res, expected);
    }
}
