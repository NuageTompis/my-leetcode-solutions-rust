struct Solution;

impl Solution {
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let mut states = vec![0; graph.len()];
        for i in 0..graph.len() {
            Self::recur(&graph, i, &mut states);
        }
        states
            .iter()
            .enumerate()
            .filter_map(|(i, x)| if *x == 2 { Some(i as i32) } else { None })
            .collect()
    }

    pub fn recur(graph: &Vec<Vec<i32>>, ndx: usize, states: &mut Vec<i32>) -> i32 {
        if states[ndx] != 0 {
            return states[ndx];
        }

        states[ndx] = 1;

        for k in &graph[ndx] {
            if states[*k as usize] == 1 || Self::recur(graph, *k as usize, states) != 2 {
                return 1;
            }
        }

        states[ndx] = 2;
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let graph: Vec<Vec<i32>> = vec![
            vec![1, 2],
            vec![2, 3],
            vec![5],
            vec![0],
            vec![5],
            vec![],
            vec![],
        ];
        let res = Solution::eventual_safe_nodes(graph);
        let expected: Vec<i32> = vec![2, 4, 5, 6]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let graph: Vec<Vec<i32>> =
            vec![vec![1, 2, 3, 4], vec![1, 2], vec![3, 4], vec![0, 4], vec![]];
        let res = Solution::eventual_safe_nodes(graph);
        let expected: Vec<i32> = vec![4]; // Fill in this value
        assert_eq!(res, expected);
    }
}
