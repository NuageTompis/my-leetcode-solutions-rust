struct Solution;

impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut ret = 0_i64;

        for pair in queries {
            let mut l = pair[0];
            let mut r = pair[1];
            let mut cpt_l = 0;
            while l > 0 {
                l >>= 2;
                cpt_l += 1;
            }
            let power_of_4_above = Self::four_pow(cpt_l);
            // println!("l={}, cpt_l={cpt_l}, power={}", pair[0], power_of_4_above);

            let mut cpt_r = 0;
            while r > 0 {
                r >>= 2;
                cpt_r += 1;
            }
            let power_of_4_below = Self::four_pow(cpt_r - 1);
            // println!("r={}, cpt_r={cpt_r}, power={}", pair[1], power_of_4_below);

            let query_operetions = if cpt_l == cpt_r {
                (pair[1] - pair[0] + 1) as i64 * cpt_l as i64
            } else {
                // println!("adding: {}", (power_of_4_above - pair[0]) as i64 * cpt_l);
                let mut operations = (power_of_4_above - pair[0] as i64) * cpt_l as i64;
                // println!(
                //     "adding: {}",
                //     (pair[1] - power_of_4_below + 1) as i64 * cpt_r
                // );
                operations += (pair[1] as i64 - power_of_4_below + 1) * cpt_r as i64;
                // for every embeddded interval add the sum
                for k in cpt_l + 1..cpt_r {
                    operations += (Self::four_pow(k) - Self::four_pow(k - 1)) * k as i64;
                }
                // ret += (Self::four_pow(cpt_l) + 3 * (cpt_r - 1) as i64 * Self::four_pow(cpt_r - 1)
                //     - Self::four_pow(cpt_r - 1))
                //     / 3
                //     - Self::four_pow(cpt_l - 1) * cpt_l as i64;
                operations
            };
            ret += (query_operetions + 1) >> 1;
        }

        ret
    }
    fn four_pow(x: i32) -> i64 {
        1 << (x << 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let queries: Vec<Vec<i32>> = vec![vec![1, 2], vec![2, 4]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let queries: Vec<Vec<i32>> = vec![vec![2, 6]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let queries: Vec<Vec<i32>> = vec![vec![3, 16]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 14; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_4() {
        let queries: Vec<Vec<i32>> = vec![vec![14, 15]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_5() {
        let queries: Vec<Vec<i32>> = vec![vec![5, 260]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 480; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_6() {
        let queries: Vec<Vec<i32>> = vec![vec![3, 64]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 87; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_7() {
        let queries: Vec<Vec<i32>> = vec![vec![3, 7], vec![3, 7]];
        let res = Solution::min_operations(queries);
        let expected: i64 = 10; // Fill in this value
        assert_eq!(res, expected);
    }
}
