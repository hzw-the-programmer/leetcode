use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            {
                let mut b = node.borrow_mut();
                (b.left, b.right) = (invert_tree(b.right.take()), invert_tree(b.left.take()));
            }
            Some(node)
        }
    }
}

// pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//     match root {
//         None => None,
//         Some(node) => {
//             let node = node.borrow();
//             Some(Rc::new(RefCell::new(TreeNode {
//                 val: node.val,
//                 left: invert_tree(node.right.clone()),
//                 right: invert_tree(node.left.clone()),
//             })))
//         }
//     }
// }
