use crate::utils::binary_tree::TreeNode;
use core::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    match root {
        None => {}
        Some(root) => {
            if root.borrow().val == x || root.borrow().val == y {
                return false;
            }

            let mut queue = VecDeque::new();
            queue.push_back(root);
            while !queue.is_empty() {
                let (mut find_x, mut find_y) = (false, false);
                for _ in 0..queue.len() {
                    let n = queue.pop_front().unwrap();
                    let iter = n
                        .borrow()
                        .left
                        .clone()
                        .into_iter()
                        .chain(n.borrow().right.clone());

                    let (mut fx, mut fy) = (false, false);
                    iter.for_each(|n| {
                        let val = n.borrow().val;
                        (fx, fy) = (fx || val == x, fy || val == y);
                        queue.push_back(n);
                    });
                    if fx && fy {
                        return false;
                    }
                    (find_x, find_y) = (find_x || fx, find_y || fy);
                    if find_x && find_y {
                        return true;
                    }
                }
            }
        }
    }

    false
}
