struct Solution;

impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();

        // index [i][j] -> answer for sub-polygon from i to j (i+2>=j)
        let mut subproblems = vec![vec![0; n]; n - 1];
        for dist in 2..n {
            for i in 0..n - dist {
                let mut best = i32::MAX;
                let j = i + dist;
                let common_edge = values[i] * values[j];
                for k in i + 1..j {
                    let sum = common_edge * values[k] + subproblems[i][k] + subproblems[k][j];
                    best = best.min(sum);
                }
                subproblems[i][j] = best;
            }
        }

        subproblems[0][n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let values: Vec<i32> = vec![1, 2, 3];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 6; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let values: Vec<i32> = vec![3, 7, 4, 5];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 144; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let values: Vec<i32> = vec![1, 3, 1, 4, 1, 5];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let values: Vec<i32> = vec![
            16, 43, 87, 30, 4, 98, 12, 30, 47, 45, 32, 4, 64, 14, 24, 84, 86, 51, 11, 22,
        ];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 118580; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let values: Vec<i32> = vec![1, 3, 1, 4, 1];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 8; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_6() {
        let values: Vec<i32> = vec![
            20, 3, 42, 70, 54, 40, 54, 65, 48, 93, 86, 100, 75, 100, 24, 3, 46, 54, 82, 11, 94, 33,
            75, 32, 20, 35, 49, 47, 46, 96, 43, 76, 38, 38, 51, 86, 5, 19, 30, 73, 66, 2, 55, 23,
            32, 13, 86, 100, 95, 24,
        ];
        let res = Solution::min_score_triangulation(values);
        let expected: i32 = 271678; // Fill in this value
        assert_eq!(res, expected);
    }
}
