use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        let mut queue = VecDeque::new();
        queue.push_back(nums[0]);
        for &x in nums.iter().skip(1) {
            let curr = queue.pop_back().unwrap();
            let gcd_ = gcd(curr, x);
            if gcd_ != 1 {
                let mut lcm = (curr / gcd_) * x;
                // propagate left
                while let Some(e) = queue.pop_back() {
                    let gcd_ = gcd(e, lcm);
                    if gcd_ != 1 {
                        lcm *= e / gcd_;
                    } else {
                        queue.push_back(e);
                        break;
                    }
                }
                queue.push_back(lcm);
            } else {
                queue.push_back(curr);
                queue.push_back(x);
            }
        }

        queue.into()
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![6, 4, 3, 2, 7, 6, 2];
        let res = Solution::replace_non_coprimes(nums);
        let expected: Vec<i32> = vec![12, 7, 6]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![2, 2, 1, 1, 3, 3, 3];
        let res = Solution::replace_non_coprimes(nums);
        let expected: Vec<i32> = vec![2, 1, 1, 3]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![2, 6];
        let res = Solution::replace_non_coprimes(nums);
        let expected: Vec<i32> = vec![6]; // Fill in this value
        assert_eq!(res, expected);
    }
}
