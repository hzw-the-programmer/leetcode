use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match (root1, root2) {
        (None, None) => None,
        (Some(root1), None) => Some(root1),
        (None, Some(root2)) => Some(root2),
        (Some(root1), Some(root2)) => {
            root1.borrow_mut().val += root2.borrow().val;
            let left = root1.borrow_mut().left.take();
            root1.borrow_mut().left = merge_trees(left, root2.borrow_mut().left.take());
            let right = root1.borrow_mut().right.take();
            root1.borrow_mut().right = merge_trees(right, root2.borrow_mut().right.take());
            Some(root1)
        }
    };
    root
}
