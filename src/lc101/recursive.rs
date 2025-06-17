use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    recursive(root.as_deref(), root.as_deref())
}

fn recursive(root1: Option<&RefCell<TreeNode>>, root2: Option<&RefCell<TreeNode>>) -> bool {
    match (root1, root2) {
        (None, None) => true,
        (Some(_), None) | (None, Some(_)) => false,
        (Some(node1), Some(node2)) => {
            let (node1, node2) = (node1.borrow(), node2.borrow());
            node1.val == node2.val
                && recursive(node1.left.as_deref(), node2.right.as_deref())
                && recursive(node1.right.as_deref(), node2.left.as_deref())
        }
    }
}

// use crate::utils::binary_tree::*;

// pub fn is_symmetric(root: Tree) -> bool {
//     if let Some(node) = &root {
//         let node = node.borrow();
//         recursive(&node.left, &node.right)
//     } else {
//         true
//     }
// }

// fn recursive(l: &Tree, r: &Tree) -> bool {
//     match (l, r) {
//         (Some(l), Some(r)) => {
//             let (l, r) = (l.borrow(), r.borrow());
//             l.val == r.val && recursive(&l.left, &r.right) && recursive(&l.right, &r.left)
//         }
//         (None, None) => true,
//         _ => false,
//     }
// }
