use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    #[allow(dead_code)]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[allow(dead_code)]
pub fn to_tree(values: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    // place seen nodes in a queue to assign children when their time comes
    let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::new();
    queue.push_back(root.clone());

    // process subsequent values in chunks of 2 (left and right children of current parent)
    for children_chunk in values[1..].chunks(2) {
        let parent = queue.pop_front().unwrap();

        // assign left child if present
        if let Some(left_value) = children_chunk.first().unwrap() {
            let left_node = Rc::new(RefCell::new(TreeNode::new(*left_value)));
            parent.borrow_mut().left = Some(left_node.clone());
            queue.push_back(left_node);
        }

        // assign right child if present
        if let Some(Some(right_value)) = children_chunk.get(1) {
            let right_node = Rc::new(RefCell::new(TreeNode::new(*right_value)));
            parent.borrow_mut().right = Some(right_node.clone());
            queue.push_back(right_node);
        }
    }

    Some(root)
}

/// ### Description
///
/// Creates a TreeNode from an array provided by leetcode.
///
/// In the given array, the pair of elements at indices (`2n+1`, `2n+2`) represents the children of the node at index `n`
///
/// ### Panics
///
/// This code will panic if the input is incorrect
/// ```
/// let will_panic = tree![1,null,null,2] // will panic since the 4th node cannot be a child of a None variant
/// ```
///
/// ### Example
///
/// Consider this tree:
///
/// ```
///   4
///    \
///     6
/// let parsed = tree![4,null,6];
/// let tree = Rc::new(RefCell::new(TreeNode{val: 4, left: None, right: Some(Rc::new(RefCell::new(TreeNode::new(6))))}));
/// assert_eq!(parsed, Some(tree));
/// ```
///
/// ### Credits
///
/// This code comes from here: https://github.com/aylei/leetcode-rust/blob/master/src/util/tree.rs
#[macro_export]
macro_rules! tree {
    () => {
        None
    };
    ($($e:expr),*) => {
        {
            let vec = vec![$(stringify!($e)), *];
            let vec = vec.into_iter().map(|v| v.parse::<i32>().ok()).collect::<Vec<_>>();
            tree::to_tree(vec)
        }
    };
    ($($e:expr,)*) => {(tree![$($e),*])};
}
