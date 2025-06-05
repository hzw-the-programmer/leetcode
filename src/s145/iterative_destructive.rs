use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    let mut stack = vec![];
    let mut current = root;
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow_mut().left.take();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            if node.borrow().right.is_none() {
                res.push(node.borrow().val);
            } else {
                current = node.borrow_mut().right.take();
                stack.push(node);
            }
        }
    }

    res
}
