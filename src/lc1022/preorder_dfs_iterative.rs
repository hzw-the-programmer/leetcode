use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use core::ptr;
use std::rc::Rc;

// time : O(n)
// space: O(n)
pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut res = 0;
    let mut n = 0;
    let mut stack = vec![];
    let mut current = root;
    let mut pre = None;
    while !stack.is_empty() || current.is_some() {
        while let Some(node) = current {
            n = (n << 1) | node.borrow().val;
            current = node.borrow().left.clone();
            stack.push(node);
        }

        if let Some(node) = stack.last() {
            current = node.borrow().right.clone();
            if node.borrow().right.is_none()
                || (pre.is_some()
                    && ptr::eq(
                        node.borrow().right.as_deref().unwrap(),
                        pre.as_deref().unwrap(),
                    ))
            {
                if node.borrow().left.is_none() && node.borrow().right.is_none() {
                    res += n;
                }
                n >>= 1;
                pre = stack.pop();
                current = None;
            }
        }
    }
    res
}

// pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
//     let mut res = 0;
//     match root {
//         None => {}
//         Some(node) => {
//             let mut stack = vec![(node, 0)];
//             while let Some((node, mut n)) = stack.pop() {
//                 let mut current = Some(node);
//                 while let Some(node) = current.take() {
//                     let node = node.borrow();
//                     n = (n << 1) + node.val;
//                     match (node.left.clone(), node.right.clone()) {
//                         (None, None) => res += n,
//                         (left, right) => {
//                             current = left;
//                             if let Some(node) = right {
//                                 stack.push((node, n));
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     res
// }
