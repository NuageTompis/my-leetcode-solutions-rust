struct Solution;

impl Solution {
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn recur(
            graph: &Vec<Vec<i32>>,
            node: usize,
            visited: &mut Vec<i32>,
            res: &mut Vec<Vec<i32>>,
        ) {
            if node == graph.len() - 1 {
                res.push(visited.clone());
                return;
            }

            for &next in &graph[node] {
                visited.push(next);
                recur(graph, next as usize, visited, res);
                visited.pop();
            }
        }

        let mut res = Vec::new();
        recur(&graph, 0, &mut vec![0], &mut res);

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let graph: Vec<Vec<i32>> = vec![vec![1, 2], vec![3], vec![3], vec![]];
        let res = Solution::all_paths_source_target(graph);
        let expected: Vec<Vec<i32>> = vec![vec![0, 1, 3], vec![0, 2, 3]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let graph: Vec<Vec<i32>> = vec![vec![4, 3, 1], vec![3, 2, 4], vec![3], vec![4], vec![]];
        let res = Solution::all_paths_source_target(graph);
        let expected: Vec<Vec<i32>> = vec![
            vec![0, 4],
            vec![0, 3, 4],
            vec![0, 1, 3, 4],
            vec![0, 1, 2, 3, 4],
            vec![0, 1, 4],
        ]; // Fill in this value
        assert_eq!(res, expected);
    }
}
