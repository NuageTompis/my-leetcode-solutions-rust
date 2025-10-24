struct Solution;

impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let n = s.len();
        let mut arr: Vec<u8> = s.as_bytes().iter().map(|d| d - b'0').collect();

        for i in 1..=n - 2 {
            for j in 0..=n - 1 - i {
                arr[j] = (arr[j] + arr[j + 1]) % 10;
            }
        }
        arr[0] == arr[1]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "3902".into();
        let res = Solution::has_same_digits(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "34789".into();
        let res = Solution::has_same_digits(s);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "632170120338946745264008215576220".into();
        let res = Solution::has_same_digits(s);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }
}
