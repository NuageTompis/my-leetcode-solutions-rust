struct Solution;

impl Solution {
    pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        let mut sum = 0;

        for c in connections {
            graph[c[0] as usize].push((c[1] as usize, true));
            graph[c[1] as usize].push((c[0] as usize, false));
        }

        let mut stack = vec![0];
        let mut visited = vec![false; n];
        visited[0] = true;

        while let Some(k) = stack.pop() {
            for &(node, to_flip) in graph[k].iter() {
                if !visited[node] {
                    if to_flip {
                        sum += 1;
                    }
                    stack.push(node);
                    visited[node] = true;
                }
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 6;
        let connections: Vec<Vec<i32>> =
            vec![vec![0, 1], vec![1, 3], vec![2, 3], vec![4, 0], vec![4, 5]];
        let res = Solution::min_reorder(n, connections);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 5;
        let connections: Vec<Vec<i32>> = vec![vec![1, 0], vec![1, 2], vec![3, 2], vec![3, 4]];
        let res = Solution::min_reorder(n, connections);
        let expected: i32 = 2; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let n: i32 = 3;
        let connections: Vec<Vec<i32>> = vec![vec![1, 0], vec![2, 0]];
        let res = Solution::min_reorder(n, connections);
        let expected: i32 = 0; // Fill in this value
        assert_eq!(res, expected);
    }
}
