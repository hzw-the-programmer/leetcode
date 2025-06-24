use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut current = root.clone();
    let mut stack = vec![];
    let mut sum = 0;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow_mut().right.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            sum += node.borrow().val;
            node.borrow_mut().val = sum;
            current = node.borrow_mut().left.clone();
        }
    }
    root
}
