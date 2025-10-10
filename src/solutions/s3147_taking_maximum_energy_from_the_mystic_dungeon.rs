struct Solution;

impl Solution {
    pub fn maximum_energy(mut energy: Vec<i32>, k: i32) -> i32 {
        let mut best = i32::MIN;
        let mut acc = vec![0; k as usize];
        energy.reverse();
        for chunk in energy.chunks(k as usize) {
            for (i, x) in chunk.iter().enumerate() {
                acc[i] += x;
                best = best.max(acc[i]);
            }
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let energy: Vec<i32> = vec![5, 2, -10, -5, 1];
        let k: i32 = 3;
        let res = Solution::maximum_energy(energy, k);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let energy: Vec<i32> = vec![-2, -3, -1];
        let k: i32 = 2;
        let res = Solution::maximum_energy(energy, k);
        let expected: i32 = -1; // Fill in this value
        assert_eq!(res, expected);
    }
}
