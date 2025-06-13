use crate::utils::binary_tree::*;

pub fn level_order_bottom(root: Tree) -> Vec<Vec<i32>> {
    let mut res = std::collections::VecDeque::new();

    if let Some(node) = root {
        let mut que = std::collections::VecDeque::new();
        que.push_back(node);
        while !que.is_empty() {
            let mut v = Vec::with_capacity(que.len());
            for _ in 0..que.len() {
                let node = que.pop_front().unwrap();
                let mut node = node.borrow_mut();
                v.push(node.val);
                if let Some(node) = node.left.take() {
                    que.push_back(node);
                }
                if let Some(node) = node.right.take() {
                    que.push_back(node);
                }
            }
            res.push_front(v);
        }
    }

    res.into()
}
