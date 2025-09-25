struct Solution;

impl Solution {
    pub fn find_circle_num(is_connected: Vec<Vec<i32>>) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let n = is_connected.len();
        let mut provinces = 0;
        for i in 0..n {
            if !seen.contains(&i) {
                provinces += 1;
                let mut to_add = vec![i];

                while let Some(k) = to_add.pop() {
                    if !seen.contains(&k) {
                        seen.insert(k);
                        for j2 in 0..n {
                            if j2 != k && is_connected[k][j2] == 1 {
                                to_add.push(j2);
                            }
                        }
                    }
                }
            }
        }
        provinces
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let is_connected: Vec<Vec<i32>> = vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]];
        let res = Solution::find_circle_num(is_connected);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let is_connected: Vec<Vec<i32>> = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        let res = Solution::find_circle_num(is_connected);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
