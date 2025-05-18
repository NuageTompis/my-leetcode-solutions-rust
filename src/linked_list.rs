#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[allow(dead_code)]
pub fn to_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut current = None;
    for &v in vec.iter().rev() {
        let mut node = ListNode::new(v);
        node.next = current;
        current = Some(Box::new(node));
    }
    current
}


/// ### Description
///
/// Creates a `[LinkedList]` from an array provided by leetcode.
///
/// ### Example
///
/// ```
/// let parsed = linked![1,4];
/// let list = Box::new(ListNode{val: 1, next: Some(Box::new(ListNode::new(4)))});
/// assert_eq!(parsed, Some(list));
/// ```
///
/// ### Credits
///
/// This code comes from [this source](https://github.com/aylei/leetcode-rust/blob/master/src/util/linked_list.rs)
#[macro_export]
macro_rules! linked {
    ($($e:expr),*) => {$crate::linked_list::to_list(vec![$($e.to_owned()), *])};
    ($($e:expr,)*) => {(linked![$($e),*])};
}