use crate::utils::binary_tree::*;

pub fn zigzag_level_order(root: Tree) -> Vec<Vec<i32>> {
    let mut res = vec![];

    if let Some(node) = root {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            let mut v = Vec::with_capacity(queue.len());
            for _ in 0..queue.len() {
                let node;
                if res.len() % 2 == 0 {
                    node = queue.pop_front().unwrap();
                } else {
                    node = queue.pop_back().unwrap();
                }
                let mut node = node.borrow_mut();
                v.push(node.val);
                if res.len() % 2 == 0 {
                    if let Some(node) = node.left.take() {
                        queue.push_back(node);
                    }
                    if let Some(node) = node.right.take() {
                        queue.push_back(node);
                    }
                } else {
                    if let Some(node) = node.right.take() {
                        queue.push_front(node);
                    }
                    if let Some(node) = node.left.take() {
                        queue.push_front(node);
                    }
                }
            }
            res.push(v);
        }
    }

    res
}
