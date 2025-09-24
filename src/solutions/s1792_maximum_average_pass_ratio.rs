#[derive(Debug, PartialEq)]
struct OrderedFloat(f32);

impl PartialOrd for OrderedFloat {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
struct Solution;

#[cfg(test)]
impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = std::collections::BinaryHeap::new();
        for c in &classes {
            let (p, t) = (c[0], c[1]);
            heap.push((
                OrderedFloat((t - p) as f32 / (t as f32 * (t as f32 + 1.0))),
                p,
                t,
            ));
        }

        for _ in 0..extra_students {
            let (_, p, t) = heap.pop().unwrap();
            let (p, t) = (p + 1, t + 1);
            heap.push((
                OrderedFloat((t - p) as f32 / (t as f32 * (t as f32 + 1.0))),
                p,
                t,
            ));
        }
        heap.iter()
            .fold(0.0, |acc, el| acc + el.1 as f64 / el.2 as f64)
            / classes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let classes: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 5], vec![2, 2]];
        let extra_students: i32 = 2;
        let res = Solution::max_average_ratio(classes, extra_students);
        let expected: f64 = 0.7833333333333333; // Fill in this value
        assert_eq!(res, expected);
    }

    // #[test]
    // fn example_2() {
    //     let classes: Vec<Vec<i32>> = vec![vec![2, 4], vec![3, 9], vec![4, 5], vec![2, 10]];
    //     let extra_students: i32 = 4;
    //     let res = Solution::max_average_ratio(classes, extra_students);
    //     let expected: f64 = 0.0; // Fill in this value
    //     assert_eq!(res, expected);
    // }
}
