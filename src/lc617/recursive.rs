use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(min(m, n))
// space: O(mind(m, n))
pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match (root1, root2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(node1), Some(node2)) => {
            {
                let (mut node1, mut node2) = (node1.borrow_mut(), node2.borrow_mut());
                node1.val += node2.val;
                node1.left = merge_trees(node1.left.take(), node2.left.take());
                node1.right = merge_trees(node1.right.take(), node2.right.take());
            }
            Some(node1)
        }
    };
    root
}
