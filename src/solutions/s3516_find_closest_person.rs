use std::cmp::Ordering;

struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        let (dx, dy) = ((z - x).abs(), (z - y).abs());
        match dx.cmp(&dy) {
            Ordering::Greater => 2,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let x: i32 = 2;
        let y: i32 = 7;
        let z: i32 = 4;
        let res = Solution::find_closest(x, y, z);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let x: i32 = 2;
        let y: i32 = 5;
        let z: i32 = 6;
        let res = Solution::find_closest(x, y, z);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let x: i32 = 1;
        let y: i32 = 5;
        let z: i32 = 3;
        let res = Solution::find_closest(x, y, z);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
