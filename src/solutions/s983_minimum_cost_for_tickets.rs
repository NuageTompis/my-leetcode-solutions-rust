struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let n = days[days.len() - 1] as usize + 1;
        let mut dp = vec![0; n];
        let mut day_ndx = 0;
        for i in 1..n {
            if i as i32 == days[day_ndx] {
                day_ndx += 1;
                let mut best = costs[0] + dp[i - 1];
                best = best.min(costs[1] + if i >= 7 { dp[i - 7] } else { 0 });
                best = best.min(costs[2] + if i >= 30 { dp[i - 30] } else { 0 });
                dp[i] = best;
            } else {
                dp[i] = dp[i - 1];
            }
        }
        dp[n - 1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let days: Vec<i32> = vec![1, 4, 6, 7, 8, 20];
        let costs: Vec<i32> = vec![2, 7, 15];
        let res = Solution::mincost_tickets(days, costs);
        let expected: i32 = 11; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let days: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31];
        let costs: Vec<i32> = vec![2, 7, 15];
        let res = Solution::mincost_tickets(days, costs);
        let expected: i32 = 17; // Fill in this value
        assert_eq!(res, expected);
    }
}
