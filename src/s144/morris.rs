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
            let mut predecessor = node.borrow().left.clone().unwrap();
            while predecessor.borrow().right.is_some() {
                let right_node = predecessor.borrow().right.clone().unwrap();
                if std::rc::Rc::ptr_eq(&right_node, &node) {
                    break;
                }
                predecessor = right_node;
            }
            if predecessor.borrow().right.is_none() {
                res.push(node.borrow().val);
                current = node.borrow().left.clone();
                predecessor.borrow_mut().right = Some(node);
            } else {
                current = node.borrow().right.clone();
                predecessor.borrow_mut().right = None;
            }
        }
    }

    res
}
