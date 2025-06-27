use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(root) => {
            {
                let mut root = root.borrow_mut();
                if root.val == key {
                    if root.right.is_none() {
                        return root.left.take();
                    } else {
                        let right = root.right.clone().unwrap();
                        if right.borrow().left.is_none() {
                            root.val = right.borrow().val;
                            root.right = right.borrow_mut().right.take();
                        } else {
                            let mut parent = right.clone();
                            let mut left = right.borrow().left.clone().unwrap();
                            while left.borrow().left.is_some() {
                                parent = left.clone();
                                let t = left.borrow().left.clone().unwrap();
                                left = t;
                            }
                            root.val = left.borrow().val;
                            parent.borrow_mut().left = left.borrow_mut().right.take();
                        }
                    }
                } else {
                    root.left = delete_node(root.left.take(), key);
                    root.right = delete_node(root.right.take(), key);
                }
            }
            Some(root)
        }
    }
}
