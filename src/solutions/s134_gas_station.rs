struct Solution;

impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut ndx = 0;
        let mut fuel = 0; // accumulative
        let n = gas.len();
        let mut total = 0;

        while ndx < n {
            let mut success = true;
            #[allow(clippy::mut_range_bound)]
            for i in ndx..n {
                fuel += gas[i] - cost[i];
                if fuel < 0 {
                    total += fuel;
                    fuel = 0;
                    // the allow clippy attribute is because clippy thinks I expected to change the bound of the for loop here, but it is actually the while loop
                    ndx = i + 1;
                    success = false;
                    break;
                }
            }
            if success {
                total += fuel;
                break;
            }
        }

        if total < 0 {
            -1
        } else {
            ndx as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let gas: Vec<i32> = vec![1, 2, 3, 4, 5];
        let cost: Vec<i32> = vec![3, 4, 5, 1, 2];
        let res = Solution::can_complete_circuit(gas, cost);
        let expected: i32 = 3; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let gas: Vec<i32> = vec![2, 3, 4];
        let cost: Vec<i32> = vec![3, 4, 3];
        let res = Solution::can_complete_circuit(gas, cost);
        let expected: i32 = -1; // Fill in this value
        assert_eq!(res, expected);
    }
}
