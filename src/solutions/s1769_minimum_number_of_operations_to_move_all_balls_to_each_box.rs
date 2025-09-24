struct Solution;

impl Solution {
    pub fn min_operations(boxes: String) -> Vec<i32> {
        let mut left = (0, 0); // amount of (balls, steps to move the balls on the left of ndx to ndx)
        let mut right = (0, 0); // idem for right
        let boxes: Vec<bool> = boxes.chars().map(|c| c == '1').collect();

        for (i, b) in boxes.iter().enumerate() {
            if *b {
                right.0 += 1;
                right.1 += i;
            }
        }

        let l = boxes.len();
        let mut res = vec![0; l];

        for i in 0..l {
            if boxes[i] {
                right.0 -= 1;
                left.0 += 1;
            }
            res[i] = (left.1 + right.1) as i32;
            left.1 += left.0;
            right.1 -= right.0;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let boxes: String = "110".into();
        let res = Solution::min_operations(boxes);
        let expected: Vec<i32> = vec![1, 1, 3]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let boxes: String = "001011".into();
        let res = Solution::min_operations(boxes);
        let expected: Vec<i32> = vec![11, 8, 5, 4, 3, 4]; // Fill in this value
        assert_eq!(res, expected);
    }
}
