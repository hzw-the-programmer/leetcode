use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    let mut stack = vec![];
    let mut current = root;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            current = node.borrow_mut().right.take();
        }
    }

    res
}
