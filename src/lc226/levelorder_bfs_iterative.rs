use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            let mut queue = VecDeque::new();
            queue.push_front(node.clone());
            while let Some(node) = queue.pop_front() {
                let mut node = node.borrow_mut();
                (node.left, node.right) = (node.right.take(), node.left.take());
                for node in node.left.clone().into_iter().chain(node.right.clone()) {
                    queue.push_front(node);
                }
            }
            Some(node)
        }
    }
}
