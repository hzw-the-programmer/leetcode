use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    recursive(root, &mut 0)
}

fn recursive(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(root) => {
            {
                let mut node = root.borrow_mut();
                node.right = recursive(node.right.take(), pre);
                node.val += *pre;
                *pre = node.val;
                node.left = recursive(node.left.take(), pre);
            }
            Some(root)
        }
    }
}
