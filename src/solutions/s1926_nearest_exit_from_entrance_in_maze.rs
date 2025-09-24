use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let n = maze.len();
        let m = maze[0].len();
        let mut visited = vec![vec![false; m]; n];
        let mut to_visit = VecDeque::new();
        to_visit.push_back((entrance[0] as usize, entrance[1] as usize, 0));

        while let Some((from_i, from_j, dist)) = to_visit.pop_front() {
            if visited[from_i][from_j] {
                continue;
            }
            visited[from_i][from_j] = true;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if (di ^ dj) & 1 == 1 {
                        let i = from_i as i32 + di;
                        let j = from_j as i32 + dj;
                        // if not out of bounds
                        if !(i < 0 || j < 0 || i >= n as i32 || j >= m as i32) {
                            let i = i as usize;
                            let j = j as usize;
                            // if the cell is unvisited and reachable
                            if maze[i][j] != '+' && !visited[i][j] {
                                if i == 0 || j == 0 || i == n - 1 || j == m - 1 {
                                    return dist + 1;
                                } else {
                                    to_visit.push_back((i, j, dist + 1));
                                }
                            }
                        }
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let maze: Vec<Vec<char>> = vec![
            vec!['+', '+', '.', '+'],
            vec!['.', '.', '.', '+'],
            vec!['+', '+', '+', '.'],
        ];
        let entrance: Vec<i32> = vec![1, 2];
        let res = Solution::nearest_exit(maze, entrance);
        let expected: i32 = 1; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let maze: Vec<Vec<char>> = vec![
            vec!['+', '+', '+'],
            vec!['.', '.', '.'],
            vec!['+', '+', '+'],
        ];
        let entrance: Vec<i32> = vec![1, 0];
        let res = Solution::nearest_exit(maze, entrance);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let maze: Vec<Vec<char>> = vec![vec!['.', '+']];
        let entrance: Vec<i32> = vec![0, 0];
        let res = Solution::nearest_exit(maze, entrance);
        let expected: i32 = -1; // Fill in this value
        assert_eq!(res, expected);
    }
}
