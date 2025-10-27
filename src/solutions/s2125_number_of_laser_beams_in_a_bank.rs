struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let bank: Vec<i32> = bank
            .iter()
            .map(|s| {
                s.as_bytes()
                    .iter()
                    .fold(0, |acc, &c| acc + if c == b'1' { 1 } else { 0 })
            })
            .filter(|&amt| amt != 0) // remove empty
            .collect();
        bank.windows(2)
            .fold(0, |acc, pair| acc + pair.iter().product::<i32>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let bank: Vec<String> = vec![
            "011001".into(),
            "000000".into(),
            "010100".into(),
            "001000".into(),
        ];
        let res = Solution::number_of_beams(bank);
        let expected: i32 = 8; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let bank: Vec<String> = vec!["000".into(), "111".into(), "000".into()];
        let res = Solution::number_of_beams(bank);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
