use crate::tree::TreeNode;
struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut heap = std::collections::BinaryHeap::new();

        let mut layer = vec![root.unwrap()];

        while !layer.is_empty() {
            let mut new_layer = Vec::new();
            let mut sum = 0;
            for node in layer {
                // add node.val
                sum += node.borrow().val as i64;
                if let Some(left) = &node.borrow().left {
                    new_layer.push(Rc::clone(left));
                }
                if let Some(right) = &node.borrow().right {
                    new_layer.push(Rc::clone(right));
                }
            }
            heap.push(sum);
            layer = new_layer;
        }

        let mut l = 0;
        while let Some(el) = heap.pop() {
            l += 1;
            if l == k {
                return el;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree;

    #[test]
    fn example_1() {
        let root: Option<Rc<RefCell<TreeNode>>> = tree![5, 8, 9, 2, 1, 3, 7, 4, 6];
        let k: i32 = 2;
        let res = Solution::kth_largest_level_sum(root, k);
        let expected: i64 = 13; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let root: Option<Rc<RefCell<TreeNode>>> = tree![1, 2, null, 3];
        let k: i32 = 1;
        let res = Solution::kth_largest_level_sum(root, k);
        let expected: i64 = 3; // Fill in this value
        assert_eq!(res, expected);
    }
}
