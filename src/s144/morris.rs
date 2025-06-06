use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];

    let mut current = root.clone();
    while let Some(node) = current {
        if node.borrow().left.is_none() {
            res.push(node.borrow().val);
            current = node.borrow().right.clone();
        } else {
            let mut rightmost = node.borrow().left.clone().unwrap();
            while rightmost.borrow().right.is_some() {
                let right = rightmost.borrow().right.clone().unwrap();
                if std::rc::Rc::ptr_eq(&right, &node) {
                    break;
                }
                rightmost = right;
            }
            if rightmost.borrow().right.is_none() {
                res.push(node.borrow().val);
                current = node.borrow().left.clone();
                rightmost.borrow_mut().right = Some(node);
            } else {
                current = node.borrow().right.clone();
                rightmost.borrow_mut().right = None;
            }
        }
    }

    res
}
