struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        // i => (min_j, max_j).
        let n = n as usize;
        let buildings: Vec<Vec<usize>> = buildings
            .iter()
            .map(|point| point.iter().map(|&x| x as usize - 1).collect())
            .collect();
        let mut line_bounds = vec![[usize::MAX, usize::MIN]; n];
        let mut column_bounds = vec![[usize::MAX, usize::MIN]; n];
        for point in &buildings {
            // point= [i,j]
            let (i, j) = (point[0], point[1]);
            line_bounds[i][0] = j.min(line_bounds[i][0]);
            line_bounds[i][1] = j.max(line_bounds[i][1]);
            column_bounds[j][0] = i.min(column_bounds[j][0]);
            column_bounds[j][1] = i.max(column_bounds[j][1]);
        }

        let mut acc = 0;
        for point in &buildings {
            let (i, j) = (point[0], point[1]);
            // (i,j) -> check line[i] and column[j]
            if !(line_bounds[i].contains(&j) || column_bounds[j].contains(&i)) {
                acc += 1;
            }
        }
        acc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 3;
        let buildings: Vec<Vec<i32>> =
            vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]];
        let res = Solution::count_covered_buildings(n, buildings);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 3;
        let buildings: Vec<Vec<i32>> = vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]];
        let res = Solution::count_covered_buildings(n, buildings);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let n: i32 = 5;
        let buildings: Vec<Vec<i32>> =
            vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]];
        let res = Solution::count_covered_buildings(n, buildings);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
