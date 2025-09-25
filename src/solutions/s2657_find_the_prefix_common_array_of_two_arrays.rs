struct Solution;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let n = a.len();
        let mut res = vec![0; n];

        let mut bits_a: u64 = 0;
        let mut bits_b: u64 = 0;

        let mut common = 0;
        let mut cpt = 0;
        for i in 0..n {
            bits_a |= 1 << a[i];
            let mut new_common = bits_a & bits_b;
            if common != new_common {
                cpt += 1;
                common = new_common;
            }
            bits_b |= 1 << b[i];
            new_common = bits_a & bits_b;
            if common != new_common {
                cpt += 1;
                common = new_common;
            }
            res[i] = cpt;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let _a: Vec<i32> = vec![1, 3, 2, 4];
        let _b: Vec<i32> = vec![3, 1, 2, 4];
        let res = Solution::find_the_prefix_common_array(_a, _b);
        let expected: Vec<i32> = vec![0, 2, 3, 4]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let _a: Vec<i32> = vec![2, 3, 1];
        let _b: Vec<i32> = vec![3, 1, 2];
        let res = Solution::find_the_prefix_common_array(_a, _b);
        let expected: Vec<i32> = vec![0, 1, 3]; // Fill in this value
        assert_eq!(res, expected);
    }
}
