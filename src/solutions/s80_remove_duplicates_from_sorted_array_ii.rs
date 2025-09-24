struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut [i32]) -> i32 {
        let mut ndc: Vec<usize> = Vec::new();

        if nums.is_empty() {
            return 0;
        }

        let mut last = nums[0];
        let mut streak = 1;
        for (i, &x) in nums.iter().enumerate().skip(1) {
            if x == last {
                streak += 1;
                if streak > 2 {
                    ndc.push(i);
                }
            } else {
                streak = 1;
                last = x;
            }
        }

        let mut slide = 0;
        let mut ndx = 0;
        for i in 0..nums.len() {
            if ndx < ndc.len() && i > ndc[ndx] {
                slide += 1;
                ndx += 1;
            }

            nums[i - slide] = nums[i];
        }

        (nums.len() - ndc.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut nums: Vec<i32> = vec![1, 1, 1, 2, 2, 3];
        let res = Solution::remove_duplicates(&mut nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
        let expected = vec![1, 1, 2, 2, 3];
        for (&a, b) in nums.iter().zip(expected) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn example_2() {
        let mut nums: Vec<i32> = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let res = Solution::remove_duplicates(&mut nums);
        let expected: i32 = 7; // Fill in this value
        assert_eq!(res, expected);
        let expected = vec![0, 0, 1, 1, 2, 3, 3];
        for (&a, b) in nums.iter().zip(expected) {
            assert_eq!(a, b);
        }
    }
}
