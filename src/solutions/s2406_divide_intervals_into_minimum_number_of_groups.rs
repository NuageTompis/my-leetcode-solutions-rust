struct Solution;

impl Solution {
    pub fn min_groups(intervals: Vec<Vec<i32>>) -> i32 {
        let mut starts = Vec::with_capacity(intervals.len());
        let mut ends = Vec::with_capacity(intervals.len());

        for el in &intervals {
            starts.push(el[0]);
            ends.push(el[1]);
        }
        starts.sort_unstable();
        ends.sort_unstable();

        let mut groups = 0;
        let mut ndx_e = 0;

        for ndx_s in starts {
            if ndx_s > ends[ndx_e] {
                ndx_e += 1;
            } else {
                groups += 1
            }
        }

        groups
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals: Vec<Vec<i32>> =
            vec![vec![5, 10], vec![6, 8], vec![1, 5], vec![2, 3], vec![1, 10]];
        let res = Solution::min_groups(intervals);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let intervals: Vec<Vec<i32>> = vec![vec![1, 3], vec![5, 6], vec![8, 10], vec![11, 13]];
        let res = Solution::min_groups(intervals);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
