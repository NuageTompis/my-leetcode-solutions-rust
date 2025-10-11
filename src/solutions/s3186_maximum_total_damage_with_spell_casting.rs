struct Solution;

impl Solution {
    pub fn maximum_total_damage(mut power: Vec<i32>) -> i64 {
        power.sort_unstable();
        let power = Solution::aggregate(power);
        let jump_indices = Solution::get_jump_indices(&power);

        let mut res = 0;
        let mut from_ndx = 0;
        for i in jump_indices {
            res += Solution::solve_for_close_group(&power[from_ndx..=i]);
            from_ndx = i + 1;
        }
        res += Solution::solve_for_close_group(&power[from_ndx..]);

        res
    }

    /// Solves the problem for a pre-processed array which is sorted,
    /// aggregated, and any pair of elements differ by at most 2
    fn solve_for_close_group(power: &[(i32, i64)]) -> i64 {
        // dp[i] = (a, b) where a is the best option until the node if we pick it,
        // and b is the best option wether or not we pick it
        let mut dp = vec![(power[0].1, power[0].1)];
        if power.len() >= 2 {
            dp.push((power[1].1, power[1].1.max(power[0].1)));
        }

        for triplet in power.windows(3) {
            let n = dp.len();
            let last_pickable_offset = match triplet[2].0 - triplet[0].0 {
                // cannot take the last two
                2 => 3,
                // cannot take the last one
                _ => 2,
            };

            let best_option_with_node = triplet[2].1
                + if n >= last_pickable_offset {
                    dp[n - last_pickable_offset].1
                } else {
                    0
                };
            dp.push((
                best_option_with_node,
                best_option_with_node.max(dp[n - 1].1),
            ));
        }

        dp.last().unwrap().1
    }

    /// Given a sorted array, aggregates duplicates to their sum
    ///
    /// ### Example
    ///
    /// ```
    /// let power = vec![1, 1, 4];
    /// let res = aggregate(power);
    /// let expected = vec![(1, 2), (4, 4)];
    /// assert_eq!(res, expected);
    fn aggregate(power: Vec<i32>) -> Vec<(i32, i64)> {
        let mut res = Vec::with_capacity(power.len());
        let mut curr_sum = power[0] as i64;
        for pair in power.windows(2) {
            if pair[0] == pair[1] {
                curr_sum += pair[0] as i64;
            } else {
                res.push((pair[0], curr_sum));
                curr_sum = pair[1] as i64;
            }
        }
        res.push((*power.last().unwrap(), curr_sum));
        res
    }

    /// Given an afore mentioned aggregated array, gives the indices where the jump is at least 3
    fn get_jump_indices(power: &[(i32, i64)]) -> Vec<usize> {
        let windows = power.windows(2).enumerate();
        windows
            .filter(|(_, pair)| pair[1].0 - pair[0].0 > 2)
            .map(|(i, _)| i)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let power: Vec<i32> = vec![1, 1, 3, 4];
        let res = Solution::maximum_total_damage(power);
        let expected: i64 = 6; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let power: Vec<i32> = vec![7, 1, 6, 6];
        let res = Solution::maximum_total_damage(power);
        let expected: i64 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let power: Vec<i32> = vec![20, 20, 21, 5, 5, 2, 1, 3, 1, 1, 6, 8, 30];
        let res = Solution::maximum_total_damage(power);
        let expected: i64 = 91; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn test_aggregate() {
        let power_sorted = vec![1, 1, 1, 2, 3, 3];
        let res = Solution::aggregate(power_sorted);
        let expected = vec![(1, 3), (2, 2), (3, 6)];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_get_jump_indices() {
        let power = vec![(1, 0), (2, 0), (5, 0), (7, 0), (9, 0), (15, 0)];
        let res = Solution::get_jump_indices(&power);
        let expected = vec![1, 4];
        assert_eq!(res, expected);
    }
}
