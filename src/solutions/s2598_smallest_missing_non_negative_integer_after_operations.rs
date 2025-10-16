#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        let mut rem_cpt = vec![0; value as usize];
        nums.iter()
            .for_each(|x| rem_cpt[((x % value + value) % value) as usize] += 1);

        let mut min_rem = (i32::MAX, 0); // (remainder, index)
        for (i, &r) in rem_cpt.iter().enumerate() {
            if r < min_rem.0 {
                min_rem = (r, i);
            }
        }

        value * min_rem.0 + min_rem.1 as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, -10, 7, 13, 6, 8];
        let value: i32 = 5;
        let res = Solution::find_smallest_integer(nums, value);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1, -10, 7, 13, 6, 8];
        let value: i32 = 7;
        let res = Solution::find_smallest_integer(nums, value);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![0, 0, 1, 2];
        let value: i32 = 3;
        let res = Solution::find_smallest_integer(nums, value);
        let expected: i32 = 4; // Fill in this value
        assert_eq!(res, expected);
    }
}
