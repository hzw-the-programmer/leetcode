// 98. Validate Binary Search Tree

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
// pub use recursive::is_valid_bst;

// mod recursive2;
// pub use recursive2::is_valid_bst;

mod iterative;
pub use iterative::is_valid_bst;

#[cfg(test)]
mod tests;
