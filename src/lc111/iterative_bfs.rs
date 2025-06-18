use crate::utils::binary_tree::*;

pub fn min_depth(root: Tree) -> i32 {
    match root {
        Some(node) => {
            let mut res = 0;
            let mut queue = std::collections::VecDeque::new();
            queue.push_back(node);
            while !queue.is_empty() {
                res += 1;
                for _ in 0..queue.len() {
                    let node = queue.pop_front().unwrap();
                    let node = node.borrow();
                    if node.left.is_none() && node.right.is_none() {
                        return res;
                    }
                    if let Some(node) = node.left.clone() {
                        queue.push_back(node);
                    }
                    if let Some(node) = node.right.clone() {
                        queue.push_back(node);
                    }
                }
            }
            res
        }
        None => 0,
    }
}
