struct Solution;

impl Solution {
    pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
        let mut a = 1;
        while a < 10000 {
            if a == 1000 {
                a += 111
            } else if a % 100 == 0 {
                a += 11;
            } else if a % 10 == 0 {
                a += 1;
            }
            let b = n - a;
            if !has_zero(b) {
                return vec![a, b];
            }
            a += 1;
        }

        panic!()
    }
}

/// Given an integer b between 1 and 9999, returns wether its decimal representation contains a zero
///
/// b is of the form dddd where only the last 3 digits can be zero
///
/// > first: n >= 1000 && n % 1000 < 100
/// >
/// > second: n >= 100 && n % 100 < 10
/// >
/// > third: n % 10 == 0
fn has_zero(b: i32) -> bool {
    // unit digit may be zero
    if b % 10 == 0 {
        true
    }
    // tens digit may be zero
    else if b >= 100 {
        if b % 100 < 10 {
            true
        }
        // hundreds digit may be zero
        else if b >= 1000 {
            b % 1000 < 100
        } else {
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 2;
        let res = Solution::get_no_zero_integers(n);
        let expected: Vec<i32> = vec![1, 1]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 11;
        let res = Solution::get_no_zero_integers(n);
        let expected: Vec<i32> = vec![2, 9]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn test_has_zero() {
        assert!(has_zero(9001));
        assert!(has_zero(1061));
        assert!(has_zero(1650));
        assert!(has_zero(401));
        assert!(!has_zero(9221));
        assert!(!has_zero(1));
    }
}
