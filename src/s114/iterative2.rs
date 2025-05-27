use crate::utils::binary_tree::*;

pub fn flatten(root: &mut Tree) {
    let mut stack = vec![];
    stack.push(root.clone());
    let mut pre: Tree = None;
    while let Some(tree) = stack.pop() {
        if let Some(node) = tree {
            stack.push(node.borrow().right.clone());
            stack.push(node.borrow().left.clone());
            if let Some(pre_node) = pre {
                pre_node.borrow_mut().left = None;
                pre_node.borrow_mut().right = Some(node.clone());
            }
            pre = Some(node);
        }
    }
}
