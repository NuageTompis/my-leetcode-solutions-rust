struct Solution;

impl Solution {
    pub fn can_sort_array(nums: Vec<i32>) -> bool {
        let mut high_group = nums[0];
        let mut high_prev = -1;
        let mut sb = set_bits(nums[0]);
        for &x in nums.iter().skip(1) {
            let sb_n = set_bits(x);
            if sb_n != sb {
                high_prev = high_prev.max(high_group);
                sb = sb_n;
            }
            if x < high_prev {
                return false;
            }
            high_group = high_group.max(x);
        }
        true
    }
}

pub fn set_bits(mut n: i32) -> i32 {
    let mut res = 0;
    while n > 0 {
        if n & 1 > 0 {
            res += 1;
        }
        n >>= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![8, 4, 2, 30, 15];
        let res = Solution::can_sort_array(nums);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
        let res = Solution::can_sort_array(nums);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![3, 16, 8, 4, 2];
        let res = Solution::can_sort_array(nums);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
