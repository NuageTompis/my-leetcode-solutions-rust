struct Solution;

pub fn constuct_modulo_10_look_up_table() -> [[i32; 5]; 2] {
    let mut res = [[0; 5]; 2];
    for i in 0..10 {
        let rem_2 = i % 2;
        let rem_5 = i % 5;
        res[rem_2][rem_5] = i as i32;
    }
    res
}

impl Solution {
    const C_5_LUT: [[usize; 5]; 5] = [
        [1, 0, 0, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 2, 1, 0, 0],
        [1, 3, 3, 1, 0],
        [1, 4, 1, 4, 1],
    ];
    const MODULO_10_LUT: [[usize; 5]; 2] = [[0, 6, 2, 8, 4], [5, 1, 7, 3, 9]];

    pub fn triangular_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len() - 1;
        let mut res = 0;
        for (k, x) in nums.iter().enumerate() {
            let binomial_coef = Self::binomial_coef_modulo_10(k, n);
            res += binomial_coef as i32 * x;
            res %= 10;
        }

        res
    }

    fn binomial_coef_modulo_10(k: usize, n: usize) -> usize {
        let mod_2 = Self::binomial_coef_modulo_2(k, n);
        let mod_5 = Self::binomial_coef_modulo_5(k, n);
        Self::MODULO_10_LUT[mod_2][mod_5]
    }

    fn binomial_coef_modulo_2(mut k: usize, mut n: usize) -> usize {
        if n < k {
            return 0;
        }

        while n > 0 {
            let rem_k = k % 2;
            let rem_n = n % 2;
            if rem_n == 0 && rem_k == 1 {
                return 0;
            }
            k >>= 1;
            n >>= 1;
        }

        1
    }

    fn binomial_coef_modulo_5(mut k: usize, mut n: usize) -> usize {
        if n < k {
            return 0;
        }

        let mut prod = 1;
        while n > 0 {
            let rem_k = k % 5;
            let rem_n = n % 5;
            let factor = Self::C_5_LUT[rem_n][rem_k];
            prod = (prod * factor) % 5;
            k /= 5;
            n /= 5;
        }

        prod
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let res = Solution::triangular_sum(nums);
        let expected: i32 = 8; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![5];
        let res = Solution::triangular_sum(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![
            0, 3, 3, 4, 1, 2, 6, 4, 9, 3, 5, 1, 7, 7, 3, 0, 3, 2, 5, 1, 9, 0, 2, 6, 3, 9, 2, 5, 9,
            2, 6, 4, 2, 9, 7, 2, 0, 3, 0, 1, 1, 2, 7, 8, 6, 4, 4, 5,
        ];
        let res = Solution::triangular_sum(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }
}
