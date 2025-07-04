use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::rc::Rc;

// enum TraversalStep {
//     First(Rc<RefCell<TreeNode>>),
//     Second(i32),
// }

// use TraversalStep::*;

// pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
//     let mut idx = 0;
//     let mut stack = Vec::from_iter(root.map(First));
//     while let Some(step) = stack.pop() {
//         match step {
//             First(node) => {
//                 let node = node.borrow();
//                 stack.extend(node.right.clone().map(First));
//                 stack.push(Second(node.val));
//                 stack.extend(node.left.clone().map(First));
//             }
//             Second(val) => {
//                 idx += 1;
//                 if idx == k {
//                     return val;
//                 }
//             }
//         }
//     }
//     unreachable!()
// }

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut idx = 0;
    let mut current = root;
    let mut stack = vec![];
    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            current = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.pop() {
            let node = node.borrow();
            idx += 1;
            if idx == k {
                return node.val;
            }
            current = node.right.clone();
        }
    }
    unreachable!()
}
