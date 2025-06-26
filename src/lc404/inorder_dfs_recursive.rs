use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn recursive(node: Option<&RefCell<TreeNode>>, is_left: bool) -> i32 {
        match node {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                if is_left && node.left.is_none() && node.right.is_none() {
                    node.val
                } else {
                    recursive(node.left.as_deref(), true) + recursive(node.right.as_deref(), false)
                }
            }
        }
    }

    recursive(root.as_deref(), false)
}

// pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     let mut res = 0;
//     match root {
//         None => {}
//         Some(root) => {
//             let root = root.borrow();
//             if let Some(node) = root.left.clone() {
//                 let node = node.borrow();
//                 if node.left.is_none() && node.right.is_none() {
//                     res += node.val;
//                 } else {
//                     res += sum_of_left_leaves(root.left.clone());
//                 }
//             }
//             res += sum_of_left_leaves(root.right.clone());
//         }
//     }
//     res
// }
