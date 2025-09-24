struct Solution;

impl Solution {
    pub fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
        let mut subs = vec![Vec::new(); n as usize];

        let mut longest = 0;

        for (i, m) in manager.iter().enumerate() {
            if *m != -1 {
                subs[*m as usize].push(i);
            }
        }

        let mut informers = vec![(head_id as usize, 0)];

        while let Some((info, t)) = informers.pop() {
            let new_t = t + inform_time[info];
            longest = longest.max(new_t);
            for sub in &subs[info] {
                informers.push((*sub, new_t));
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 1;
        let head_id: i32 = 0;
        let manager: Vec<i32> = vec![-1];
        let inform_time: Vec<i32> = vec![0];
        let res = Solution::num_of_minutes(n, head_id, manager, inform_time);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 6;
        let head_id: i32 = 2;
        let manager: Vec<i32> = vec![2, 2, -1, 2, 2, 2];
        let inform_time: Vec<i32> = vec![0, 0, 1, 0, 0, 0];
        let res = Solution::num_of_minutes(n, head_id, manager, inform_time);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
