use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sorted = nums.clone();
        sorted.sort();

        let mut output = Vec::new();

        let n = nums.len();
        for i in 0..n - 2 {
            if i > 0 && sorted[i] == sorted[i - 1] {
                continue;
            }
            let mut j = i + 1;
            let mut k = n - 1;

            while j < k {
                let sum = sorted[i] + sorted[j] + sorted[k];
                match sum.cmp(&0) {
                    Ordering::Less => {
                        j += 1;
                    }
                    Ordering::Equal => {
                        let new_triplet = vec![sorted[i], sorted[j], sorted[k]];
                        k -= 1;
                        j += 1;
                        while j < k && sorted[j] == sorted[j - 1] {
                            j += 1;
                        }
                        while k < n - 1 && k > j && sorted[k] == sorted[k + 1] {
                            k -= 1;
                        }
                        output.push(new_triplet);
                    }
                    Ordering::Greater => {
                        k -= 1;
                    }
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![-1, 0, 1, 2, -1, -4];
        let res = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![vec![-1, -1, 2], vec![-1, 0, 1]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![0, 1, 1];
        let res = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![0, 0, 0];
        let res = Solution::three_sum(nums);
        let expected: Vec<Vec<i32>> = vec![vec![0, 0, 0]]; // Fill in this value
        assert_eq!(res, expected);
    }
}
