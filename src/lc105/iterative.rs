use crate::utils::binary_tree::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

// time  : O(n)
// space : O(n)
pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));

    let mut stack = vec![root.clone()];
    let mut index = 0;
    for &n in &preorder[1..] {
        let mut node = stack.last().unwrap().clone();
        if node.borrow().val != inorder[index] {
            let left = Rc::new(RefCell::new(TreeNode::new(n)));
            stack.push(left.clone());
            node.borrow_mut().left = Some(left);
        } else {
            while let Some(top) = stack.last() {
                if top.borrow().val != inorder[index] {
                    break;
                }
                node = stack.pop().unwrap();
                index += 1;
            }
            let right = Rc::new(RefCell::new(TreeNode::new(n)));
            stack.push(right.clone());
            node.borrow_mut().right = Some(right);
        }
    }

    Some(root)
}
