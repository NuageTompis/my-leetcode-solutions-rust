struct Solution;

impl Solution {
    pub fn largest_triangle_area(points: Vec<Vec<i32>>) -> f64 {
        let mut largest: f64 = 0.0;
        let n = points.len();
        for (i, a) in points.iter().take(n - 2).enumerate() {
            for (j, b) in points.iter().take(n - 1).skip(i + 1).enumerate() {
                for c in points.iter().skip(j + 1) {
                    let area: f64 = Self::compute_area(a, b, c);
                    largest = largest.max(area);
                }
            }
        }

        largest
    }

    fn compute_area(a: &[i32], b: &[i32], c: &[i32]) -> f64 {
        (a[0] as f64 * (b[1] - c[1]) as f64
            + b[0] as f64 * (c[1] - a[1]) as f64
            + c[0] as f64 * (a[1] - b[1]) as f64)
            .abs()
            / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let points: Vec<Vec<i32>> =
            vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![0, 2], vec![2, 0]];
        let res = Solution::largest_triangle_area(points);
        let expected: f64 = 2.0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let points: Vec<Vec<i32>> = vec![vec![1, 0], vec![0, 0], vec![0, 1]];
        let res = Solution::largest_triangle_area(points);
        let expected: f64 = 0.5; // Fill in this value
        assert_eq!(res, expected);
    }
}
