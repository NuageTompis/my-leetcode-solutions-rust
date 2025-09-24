struct Solution;

impl Solution {
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ajd = vec![Vec::new(); n as usize];
        for c in &connections {
            ajd[c[0] as usize].push(c[1] as usize);
            ajd[c[1] as usize].push(c[0] as usize);
        }

        let mut visited = vec![false; n as usize];
        let mut bridges = Vec::new();

        let mut dfs_ndc = vec![0; n as usize];
        let mut low = vec![0; n as usize];

        let mut dfs_ndx = 0;

        Self::dfs(
            0,
            usize::MAX,
            &ajd,
            &mut visited,
            &mut dfs_ndc,
            &mut low,
            &mut dfs_ndx,
            &mut bridges,
        );

        bridges
    }

    #[allow(clippy::too_many_arguments)]
    pub fn dfs(
        node: usize,
        parent: usize,
        ajd: &Vec<Vec<usize>>,
        visited: &mut Vec<bool>,
        dfs_ndc: &mut Vec<u32>,
        low: &mut Vec<u32>,
        dfs_ndx: &mut u32,
        bridges: &mut Vec<Vec<i32>>,
    ) {
        visited[node] = true;

        dfs_ndc[node] = *dfs_ndx;
        low[node] = *dfs_ndx;
        *dfs_ndx += 1;

        for child in &ajd[node] {
            if *child != parent {
                if visited[*child] {
                    low[node] = low[node].min(low[*child]);
                } else {
                    Self::dfs(*child, node, ajd, visited, dfs_ndc, low, dfs_ndx, bridges);
                    low[node] = low[node].min(low[*child]);
                    if low[*child] > dfs_ndc[node] {
                        bridges.push(vec![node as i32, *child as i32]);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n: i32 = 4;
        let connections: Vec<Vec<i32>> = vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]];
        let res = Solution::critical_connections(n, connections);
        let expected: Vec<Vec<i32>> = vec![vec![1, 3]]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let n: i32 = 2;
        let connections: Vec<Vec<i32>> = vec![vec![0, 1]];
        let res = Solution::critical_connections(n, connections);
        let expected: Vec<Vec<i32>> = vec![vec![0, 1]]; // Fill in this value
        assert_eq!(res, expected);
    }
}
