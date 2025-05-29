use crate::utils::binary_tree::*;

pub fn zigzag_level_order(root: Tree) -> Vec<Vec<i32>> {
    let mut res = vec![];

    if let Some(node) = root {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        let mut is_left = true;
        while !queue.is_empty() {
            let mut q = std::collections::VecDeque::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node = queue.pop_front().unwrap();
                let mut node = node.borrow_mut();
                if is_left {
                    q.push_back(node.val);
                } else {
                    q.push_front(node.val);
                }
                if let Some(node) = node.left.take() {
                    queue.push_back(node);
                }
                if let Some(node) = node.right.take() {
                    queue.push_back(node);
                }
            }
            is_left = !is_left;
            res.push(q.into());
        }
    }

    res
}
