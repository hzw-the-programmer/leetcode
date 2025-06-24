use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root.clone() {
        None => {}
        Some(root) => {
            root.borrow_mut().val = 0;
            let mut queue = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let sum = queue
                    .iter()
                    .flat_map(|n| {
                        n.borrow()
                            .left
                            .clone()
                            .into_iter()
                            .chain(n.borrow().right.clone())
                    })
                    .map(|n| n.borrow().val)
                    .sum::<i32>();

                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let iter = node
                        .borrow()
                        .left
                        .clone()
                        .into_iter()
                        .chain(node.borrow().right.clone());
                    let s = iter.clone().map(|n| n.borrow().val).sum::<i32>();
                    iter.for_each(|n| {
                        n.borrow_mut().val = sum - s;
                        queue.push_back(n);
                    });
                }
            }
        }
    }

    root
}

// pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
//     let mut queue = VecDeque::new();
//     queue.push_back(root.clone());
//     while !queue.is_empty() {
//         let sum = queue
//             .iter()
//             .map(|n| match n {
//                 None => 0,
//                 Some(n) => n.borrow().val,
//             })
//             .sum::<i32>();

//         for _ in (0..queue.len()).step_by(2) {
//             let (n1, n2) = match (queue.pop_front(), queue.pop_front()) {
//                 (Some(n1), Some(n2)) => (n1, n2),
//                 (Some(n1), None) => (n1, None),
//                 _ => (None, None),
//             };
//             let iter = n1.into_iter().chain(n2);
//             let s = iter.clone().map(|n| n.borrow().val).sum::<i32>();
//             for n in iter {
//                 n.borrow_mut().val = sum - s;
//                 queue.push_back(n.borrow().left.clone());
//                 queue.push_back(n.borrow().right.clone());
//             }
//         }
//     }

//     root
// }
