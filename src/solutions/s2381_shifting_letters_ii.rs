struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let l = s.len();
        let mut shift_coefs = vec![0; l + 1];

        for shift in &shifts {
            shift_coefs[shift[0] as usize] += if shift[2] == 1 { 1 } else { 25 };
            shift_coefs[shift[1] as usize + 1] -= if shift[2] == 1 { 1 } else { 25 };
        }

        let a = 'a' as i32;
        let mut res = vec!['a'; l];
        let mut sum = 0;
        for (i, c) in s.chars().enumerate() {
            sum += shift_coefs[i];
            res[i] = ((((c as i32 + sum - a) % 26 + 26) % 26) + a) as u8 as char;
        }

        res.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s: String = "abc".into();
        let shifts: Vec<Vec<i32>> = vec![vec![0, 1, 0], vec![1, 2, 1], vec![0, 2, 1]];
        let res = Solution::shifting_letters(s, shifts);
        let expected: String = "ace".into(); // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let s: String = "dztz".into();
        let shifts: Vec<Vec<i32>> = vec![vec![0, 0, 0], vec![1, 1, 1]];
        let res = Solution::shifting_letters(s, shifts);
        let expected: String = "catz".into(); // Fill in this value
        assert_eq!(res, expected);
    }
}
