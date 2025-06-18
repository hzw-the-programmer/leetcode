use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    let mut stack = vec![];
    let mut current = root.clone();
    let mut pre = ptr::null();
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            if node.borrow().right.is_some()
                && !ptr::eq(node.borrow().right.as_deref().unwrap(), pre)
            {
                current = node.borrow().right.clone();
                stack.push(node);
            } else {
                res.push(node.borrow().val);
                pre = &*node;
            }
        }
    }

    res
}
