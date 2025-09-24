struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, mut new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::with_capacity(intervals.len() + 1);

        let mut has_inserted = false;
        for curr in intervals {
            if curr[1] < new_interval[0] {
                res.push(curr);
            } else if curr[0] > new_interval[1] {
                if !has_inserted {
                    res.push(new_interval.clone());
                    has_inserted = true;
                }
                res.push(curr);
            } else {
                new_interval[0] = new_interval[0].min(curr[0]);
                new_interval[1] = new_interval[1].max(curr[1]);
            }
        }

        if !has_inserted {
            res.push(new_interval);
        }

        res
    }
}

pub fn binary_search(intervals: &[Vec<i32>], x: i32) -> (usize, bool) {
    let mut a = 0;
    let mut b = intervals.len() - 1;
    while a < b {
        let c = (a + b) >> 1;
        if intervals[c][1] < x {
            a = c + 1;
        } else {
            b = c;
        }
    }

    if a == intervals.len() - 1 && x > intervals[a][1] {
        (a + 1, false)
    } else {
        (a, x >= intervals[a][0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let intervals: Vec<Vec<i32>> = vec![vec![1, 3], vec![6, 9]];
        let new_interval: Vec<i32> = vec![2, 5];
        let res = Solution::insert(intervals, new_interval);
        let expected: Vec<Vec<i32>> = vec![vec![1, 5], vec![6, 9]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let intervals: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval: Vec<i32> = vec![4, 8];
        let res = Solution::insert(intervals, new_interval);
        let expected: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 10], vec![12, 16]]; // Fill in this value
        assert_eq!(res, expected);
    }
}
