struct Solution;

impl Solution {
    pub fn xor_all_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let l1 = nums1.len();
        let l2 = nums2.len();
        let mut cpt1 = [0; 31];
        let mut cpt2 = [0; 31];
        let mut bit = 1;
        for i in 0..31 {
            for n in &nums1 {
                if n & bit > 0 {
                    cpt1[i] += 1;
                }
            }
            for n in &nums2 {
                if n & bit > 0 {
                    cpt2[i] += 1;
                }
            }
            bit <<= 1;
        }

        let mut res = 0;
        bit = 1 << 30;

        for i in (0..31).rev() {
            if (cpt1[i] * (l2 - cpt2[i]) + cpt2[i] * (l1 - cpt1[i])) % 2 == 1 {
                res += 1;
            }
            bit >>= 1;
            res <<= 1;
        }

        res >> 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums1: Vec<i32> = vec![2, 1, 3];
        let nums2: Vec<i32> = vec![10, 2, 5, 0];
        let res = Solution::xor_all_nums(nums1, nums2);
        let expected: i32 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums1: Vec<i32> = vec![1, 2];
        let nums2: Vec<i32> = vec![3, 4];
        let res = Solution::xor_all_nums(nums1, nums2);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
