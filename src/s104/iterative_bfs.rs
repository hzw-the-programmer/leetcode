use crate::utils::binary_tree::*;

pub fn max_depth(root: Tree) -> i32 {
    if let Some(node) = root {
        let mut res = 0;
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(node);
        while !queue.is_empty() {
            res += 1;
            let n = queue.len();
            for _ in 0..n {
                let node = queue.pop_front().unwrap();
                if node.borrow().left.is_some() {
                    queue.push_back(node.borrow().left.clone().unwrap());
                }
                if node.borrow().right.is_some() {
                    queue.push_back(node.borrow().right.clone().unwrap());
                }
            }
        }
        res
    } else {
        0
    }
}
