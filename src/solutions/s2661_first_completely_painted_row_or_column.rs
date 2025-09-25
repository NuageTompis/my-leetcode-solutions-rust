struct Solution;

impl Solution {
    pub fn first_complete_index(arr: Vec<i32>, mat: Vec<Vec<i32>>) -> i32 {
        let m = mat.len();
        let n = mat[0].len();

        let mut lines = vec![0; m];
        let mut columns = vec![0; n];

        let mut map = vec![(0, 0); arr.len() + 1];

        for (i, line) in mat.iter().enumerate() {
            for (j, &k) in line.iter().enumerate() {
                map[k as usize] = (i, j);
            }
        }

        for (ndx, &k) in arr.iter().enumerate() {
            let (i, j) = map[k as usize];
            lines[i] += 1;
            columns[j] += 1;
            if lines[i] == n || columns[j] == m {
                return ndx as i32;
            }
        }

        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let arr: Vec<i32> = vec![1, 3, 4, 2];
        let mat: Vec<Vec<i32>> = vec![vec![1, 4], vec![2, 3]];
        let res = Solution::first_complete_index(arr, mat);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let arr: Vec<i32> = vec![2, 8, 7, 4, 1, 3, 5, 6, 9];
        let mat: Vec<Vec<i32>> = vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]];
        let res = Solution::first_complete_index(arr, mat);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
