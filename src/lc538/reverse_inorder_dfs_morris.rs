use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut sum = 0;
    let mut current = root.clone();
    while let Some(root) = current {
        if root.borrow().right.is_some() {
            let mut left_most = root.borrow().right.clone().unwrap();
            while left_most.borrow().left.is_some() {
                let left = left_most.borrow().left.clone().unwrap();
                if Rc::ptr_eq(&left, &root) {
                    break;
                }
                left_most = left;
            }

            if left_most.borrow().left.is_none() {
                left_most.borrow_mut().left = Some(root.clone());
                current = root.borrow().right.clone();
            } else {
                left_most.borrow_mut().left = None;
                current = root.borrow().left.clone();

                sum += root.borrow().val;
                root.borrow_mut().val = sum;
            }
        } else {
            current = root.borrow().left.clone();

            sum += root.borrow().val;
            root.borrow_mut().val = sum;
        }
    }
    root
}
