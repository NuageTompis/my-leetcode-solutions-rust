struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        let mut len = nums.len();
        while len > 1 {
            for i in 0..=(nums.len() - len) {
                let g = Self::gcd_vec(&nums[i..len + i]);
                let (prod, l) = if g == 1 {
                    let slice = &nums[i..len + i];
                    let mut l = 1;
                    let mut prod = 1;

                    for v in slice.iter() {
                        l = Self::lcm(l, *v);
                        prod *= v;
                        if l != prod {
                            break;
                        }
                    }

                    (prod, l)
                } else {
                    (
                        nums[i..len + i].iter().product(),
                        nums[i..len + i].iter().fold(1, |acc, x| Self::lcm(acc, *x)),
                    )
                };
                if g * l == prod {
                    return len as i32;
                }
            }
            len -= 1;
        }
        1
    }

    pub fn gcd_vec(nums: &[i32]) -> i32 {
        let mut res = nums[0];
        for &x in nums.iter().skip(1) {
            res = Self::gcd(res, x);
        }
        res
    }

    pub fn lcm(a: i32, b: i32) -> i32 {
        a * b / Self::gcd(a, b)
    }

    pub fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Self::gcd(b, a % b)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 1, 1];
        let res = Solution::max_length(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![2, 3, 4, 5, 6];
        let res = Solution::max_length(nums);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 4, 5, 1];
        let res = Solution::max_length(nums);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }
}
