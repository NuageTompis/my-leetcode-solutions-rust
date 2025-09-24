use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn shortest_alternating_paths(
        n: i32,
        red_edges: Vec<Vec<i32>>,
        blue_edges: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        // (node, visited_from) where visited_from=1 for red and 2 for blue
        let mut reached = vec![(0, 1), (0, 2)];

        let mut red_map = HashMap::<usize, Vec<usize>>::new();
        let mut blue_map = HashMap::<usize, Vec<usize>>::new();

        for re in &red_edges {
            red_map
                .entry(re[0] as usize)
                .or_default()
                .push(re[1] as usize);
        }
        for be in &blue_edges {
            blue_map
                .entry(be[0] as usize)
                .or_default()
                .push(be[1] as usize);
        }

        let mut shortest = vec![vec![-1, -1]; n as usize];
        shortest[0] = vec![0, 0]; // (reached from red, reached from blue)

        while let Some((i, color)) = reached.pop() {
            let map = if color == 1 { &blue_map } else { &red_map };
            if let Some(edges) = map.get(&i) {
                for e in edges {
                    if shortest[*e][color - 1] == -1
                        || shortest[*e][color - 1] > shortest[i][color % 2] + 1
                    {
                        shortest[*e][color - 1] = shortest[i][color % 2] + 1;
                        reached.push((*e, color % 2 + 1));
                    }
                }
            }
        }

        shortest
            .iter()
            .map(|v| {
                if v[0] == -1 {
                    v[1]
                } else if v[1] == -1 {
                    v[0]
                } else {
                    v[0].min(v[1])
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 3;
        let red_edges: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 2]];
        let blue_edges: Vec<Vec<i32>> = vec![];
        let res = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        let expected: Vec<i32> = vec![0, 1, -1]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 3;
        let red_edges: Vec<Vec<i32>> = vec![vec![0, 1]];
        let blue_edges: Vec<Vec<i32>> = vec![vec![2, 1]];
        let res = Solution::shortest_alternating_paths(n, red_edges, blue_edges);
        let expected: Vec<i32> = vec![0, 1, -1]; // Fill in this value
        assert_eq!(res, expected);
    }
}
