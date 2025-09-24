struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = std::collections::HashSet::new();
        let mut to_visit = vec![0];

        while let Some(i) = to_visit.pop() {
            visited.insert(i);
            for j in &rooms[i] {
                if !visited.contains(&(*j as usize)) {
                    to_visit.push(*j as usize);
                }
            }
        }

        visited.len() == rooms.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let rooms: Vec<Vec<i32>> = vec![vec![1], vec![2], vec![3], vec![]];
        let res = Solution::can_visit_all_rooms(rooms);
        let expected: bool = true; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let rooms: Vec<Vec<i32>> = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
        let res = Solution::can_visit_all_rooms(rooms);
        let expected: bool = false; // Fill in this value
        assert_eq!(res, expected);
    }
}
