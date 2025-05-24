// 94. Binary Tree Inorder Traversal

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

fn new_tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

// mod recursive;
// pub use recursive::inorder_traversal;

mod iterative;
pub use iterative::inorder_traversal;

#[cfg(test)]
mod tests;
