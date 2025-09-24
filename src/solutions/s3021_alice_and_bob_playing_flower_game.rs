#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn flower_game(n: i32, m: i32) -> i64 {
        let n_even = n as i64 >> 1;
        let n_odd = (n as i64 + 1) >> 1;
        let m_even = m as i64 >> 1;
        let m_odd = (m as i64 + 1) >> 1;
        n_even * m_odd + n_odd * m_even
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 3;
        let m: i32 = 2;
        let res = Solution::flower_game(n, m);
        let expected: i64 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 1;
        let m: i32 = 1;
        let res = Solution::flower_game(n, m);
        let expected: i64 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
