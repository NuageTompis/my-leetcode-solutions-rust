struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut cl = 0;
        let mut cr = height.len() - 1;
        let mut ca = 0;

        while cl < cr {
            let a = (cr as i32 - cl as i32) * height[cl].min(height[cr]);
            ca = ca.max(a);
            if height[cl] < height[cr] {
                cl += 1;
            } else {
                cr -= 1;
            }
        }

        ca
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let height: Vec<i32> = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let res = Solution::max_area(height);
        let expected: i32 = 49; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let height: Vec<i32> = vec![1, 1];
        let res = Solution::max_area(height);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }
}
