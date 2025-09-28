struct Solution;

impl Solution {
    pub fn decimal_representation(mut n: i32) -> Vec<i32> {
        let mut res = Vec::new();

        let mut div = 1;
        while n > 0 {
            let r = n % div;
            if r > 0 {
                res.push(r);
                n -= r;
            }
            div *= 10;
        }

        res.reverse();
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 537;
        let res = Solution::decimal_representation(n);
        let expected: Vec<i32> = vec![500, 30, 7]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 102;
        let res = Solution::decimal_representation(n);
        let expected: Vec<i32> = vec![100, 2]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let n: i32 = 6;
        let res = Solution::decimal_representation(n);
        let expected: Vec<i32> = vec![6]; // Fill in this value
        assert_eq!(res, expected);
    }
}
