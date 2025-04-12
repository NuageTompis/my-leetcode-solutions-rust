struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        0   
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex_1() {
        let nums = vec![10,20,30,5,10,50];
        assert_eq!(Solution::max_ascending_sum(nums), 65);
    }
}