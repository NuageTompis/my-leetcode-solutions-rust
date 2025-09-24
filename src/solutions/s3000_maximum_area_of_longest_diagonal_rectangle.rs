#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        let mut max_diago_sq = 0;
        let mut max_area = 0;
        for rec in dimensions {
            let diago_sq = rec[0] * rec[0] + rec[1] * rec[1];
            if diago_sq > max_diago_sq {
                max_diago_sq = diago_sq;
                max_area = rec[0] * rec[1];
            } else if diago_sq == max_diago_sq {
                max_area = max_area.max(rec[0] * rec[1]);
            }
        }
        max_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let dimensions: Vec<Vec<i32>> = vec![vec![9, 3], vec![8, 6]];
        let res = Solution::area_of_max_diagonal(dimensions);
        let expected: i32 = 48; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let dimensions: Vec<Vec<i32>> = vec![vec![3, 4], vec![4, 3]];
        let res = Solution::area_of_max_diagonal(dimensions);
        let expected: i32 = 12; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let dimensions: Vec<Vec<i32>> = vec![
            vec![4, 7],
            vec![8, 9],
            vec![5, 3],
            vec![6, 10],
            vec![2, 9],
            vec![3, 10],
            vec![2, 2],
            vec![5, 8],
            vec![5, 10],
            vec![5, 6],
            vec![8, 9],
            vec![10, 7],
            vec![8, 9],
            vec![3, 7],
            vec![2, 6],
            vec![5, 1],
            vec![7, 4],
            vec![1, 10],
            vec![1, 7],
            vec![6, 9],
            vec![3, 3],
            vec![4, 6],
            vec![8, 2],
            vec![10, 6],
            vec![7, 9],
            vec![9, 2],
            vec![1, 2],
            vec![3, 8],
            vec![10, 2],
            vec![4, 1],
            vec![9, 7],
            vec![10, 3],
            vec![6, 9],
            vec![9, 8],
            vec![7, 7],
            vec![5, 7],
            vec![5, 4],
            vec![6, 5],
            vec![1, 8],
            vec![2, 3],
            vec![7, 10],
            vec![3, 9],
            vec![5, 7],
            vec![2, 4],
            vec![5, 6],
            vec![9, 5],
            vec![8, 8],
            vec![8, 10],
            vec![6, 8],
            vec![5, 1],
            vec![10, 8],
            vec![7, 4],
            vec![2, 1],
            vec![2, 7],
            vec![10, 3],
            vec![2, 5],
            vec![7, 6],
            vec![10, 5],
            vec![10, 9],
            vec![5, 7],
            vec![10, 6],
            vec![4, 3],
            vec![10, 4],
            vec![1, 5],
            vec![8, 9],
            vec![3, 1],
            vec![2, 5],
            vec![9, 10],
            vec![6, 6],
            vec![5, 10],
            vec![10, 2],
            vec![6, 10],
            vec![1, 1],
            vec![8, 6],
            vec![1, 7],
            vec![6, 3],
            vec![9, 3],
            vec![1, 4],
            vec![1, 1],
            vec![10, 4],
            vec![7, 9],
            vec![4, 5],
            vec![2, 8],
            vec![7, 9],
            vec![7, 3],
            vec![4, 9],
            vec![2, 8],
            vec![4, 6],
            vec![9, 1],
            vec![8, 4],
            vec![2, 4],
            vec![7, 8],
            vec![3, 5],
            vec![7, 6],
            vec![8, 6],
            vec![4, 7],
            vec![25, 60],
            vec![39, 52],
            vec![16, 63],
            vec![33, 56],
        ];
        let res = Solution::area_of_max_diagonal(dimensions);
        let expected: i32 = 2028; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let dimensions: Vec<Vec<i32>> = vec![
            vec![6, 5],
            vec![8, 6],
            vec![2, 10],
            vec![8, 1],
            vec![9, 2],
            vec![3, 5],
            vec![3, 5],
        ];
        let res = Solution::area_of_max_diagonal(dimensions);
        let expected: i32 = 20; // Fill in this value
        assert_eq!(res, expected);
    }
}
