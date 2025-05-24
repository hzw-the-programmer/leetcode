use super::*;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    let mut stack = vec![];
    let mut tree = root;
    while tree.is_some() || !stack.is_empty() {
        while let Some(node) = tree {
            stack.push(node.clone());
            tree = node.borrow().left.clone();
        }

        if let Some(node) = stack.pop() {
            res.push(node.borrow().val);
            tree = node.borrow().right.clone();
        }
    }

    res
}
