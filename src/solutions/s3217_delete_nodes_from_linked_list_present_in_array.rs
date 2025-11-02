use std::collections::HashSet;

use crate::linked_list::ListNode;
struct Solution;

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn modified_list(nums: Vec<i32>, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let nums: HashSet<i32> = HashSet::from_iter(nums);
        let mut dummy = ListNode { val: 0, next: head };

        let mut curr = &mut dummy;
        while let Some(next) = curr.next.take() {
            if nums.contains(&next.val) {
                curr.next = next.next;
            } else {
                curr.next = Some(next);
                curr = curr.next.as_mut().unwrap();
            }
        }

        dummy.next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linked;

    #[test]
    fn example_1() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let head: Option<Box<ListNode>> = linked![1, 2, 3, 4, 5];
        let res = Solution::modified_list(nums, head);
        let expected: Option<Box<ListNode>> = linked![4, 5]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_2() {
        let nums: Vec<i32> = vec![1];
        let head: Option<Box<ListNode>> = linked![1, 2, 1, 2, 1, 2];
        let res = Solution::modified_list(nums, head);
        let expected: Option<Box<ListNode>> = linked![2, 2, 2]; // Fill in this value
        assert_eq!(res, expected);
    }

    #[test]
    fn example_3() {
        let nums: Vec<i32> = vec![5];
        let head: Option<Box<ListNode>> = linked![1, 2, 3, 4];
        let res = Solution::modified_list(nums, head);
        let expected: Option<Box<ListNode>> = linked![1, 2, 3, 4]; // Fill in this value
        assert_eq!(res, expected);
    }
}
