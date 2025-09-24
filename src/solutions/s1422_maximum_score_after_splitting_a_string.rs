struct Solution;

impl Solution {
    pub fn max_score(s: String) -> i32 {
        let ones: Vec<bool> = s.chars().map(|c| c == '1').collect();
        let mut score = ones.iter().fold(0, |acc, x| if *x { acc + 1 } else { acc });
        let mut best = 0;

        for &b in ones.iter().take(ones.len() - 1) {
            if b {
                score -= 1;
            } else {
                score += 1;
            }
            best = best.max(score);
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "011101".into();
        let res = Solution::max_score(s);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "00111".into();
        let res = Solution::max_score(s);
        let expected: i32 = 5; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let s: String = "1111".into();
        let res = Solution::max_score(s);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
