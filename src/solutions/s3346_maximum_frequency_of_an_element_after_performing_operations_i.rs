struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        // sort
        nums.sort_unstable();

        // two options to get max frequency:
        // 1. sliding window of size 2k
        // 2. shift close elements to existing value

        // 1. sliding window (shift *all* to center of window)
        let mut i = 0;
        let mut j = 0;
        let n = nums.len();
        let mut best = 0;
        let k2 = k << 1;
        while j < n {
            if nums[j] - nums[i] > k2 {
                i += 1;
            } else {
                best = best.max((j - i + 1) as i32);
                j += 1;
            }
        }

        best = best.min(num_operations);

        // 2. shift neighbors
        // aggregate ([1,1,1,2] -> [(1,3), (2,1)])
        let mut prev = (nums[0], 1); // (value, amt)
        let mut aggregated = Vec::new();
        for &x in nums.iter().skip(1) {
            if x == prev.0 {
                prev.1 += 1;
            } else {
                aggregated.push(prev);
                prev = (x, 1);
            }
        }
        aggregated.push(prev);

        // for each cluster, add the sum of left and right clusters than can be shifted
        let mut i = 0; // last added (inclusive)
        let mut sum_i = 0;
        let mut j = 1; // next to add (exclusive)
        let mut sum_j = aggregated[0].1; // right sum includes current
        let n = aggregated.len();
        for &(x, amt) in &aggregated {
            // bring i closer
            while x - aggregated[i].0 > k {
                sum_i -= aggregated[i].1;
                i += 1;
            }
            // push j forward
            while j < n && aggregated[j].0 - x <= k {
                sum_j += aggregated[j].1;
                j += 1;
            }
            best = best.max(amt + (sum_i + sum_j - amt).min(num_operations));

            // end iteration
            sum_i += amt;
            sum_j -= amt;
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 4, 5];
        let k: i32 = 1;
        let num_operations: i32 = 2;
        let res = Solution::max_frequency(nums, k, num_operations);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![5, 11, 20, 20];
        let k: i32 = 5;
        let num_operations: i32 = 1;
        let res = Solution::max_frequency(nums, k, num_operations);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![22, 32, 74, 90];
        let k: i32 = 39;
        let num_operations: i32 = 4;
        let res = Solution::max_frequency(nums, k, num_operations);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let nums: Vec<i32> = vec![9];
        let k: i32 = 0;
        let num_operations: i32 = 0;
        let res = Solution::max_frequency(nums, k, num_operations);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
