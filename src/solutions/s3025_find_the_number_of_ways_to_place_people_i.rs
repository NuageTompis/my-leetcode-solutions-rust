struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points: Vec<(i32, i32)> = points.iter().map(|p| (p[0], p[1])).collect();
        points.sort_by(|a, b| match b.1.cmp(&a.1) {
            std::cmp::Ordering::Equal => a.0.cmp(&b.0),
            other => other,
        });

        let mut pairs = 0;
        for (i, &(x, _)) in points.iter().enumerate() {
            let mut right = i32::MAX;
            for p in points.iter().skip(i + 1) {
                if p.0 >= right || p.0 < x {
                    continue;
                }
                right = p.0;
                pairs += 1;
            }
        }

        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points: Vec<Vec<i32>> = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
        let res = Solution::number_of_pairs(points);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let points: Vec<Vec<i32>> = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
        let res = Solution::number_of_pairs(points);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let points: Vec<Vec<i32>> = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
        let res = Solution::number_of_pairs(points);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let points: Vec<Vec<i32>> =
            vec![vec![1, 3], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 1]];
        let res = Solution::number_of_pairs(points);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let points: Vec<Vec<i32>> = vec![vec![0, 0], vec![0, 3]];
        let res = Solution::number_of_pairs(points);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
