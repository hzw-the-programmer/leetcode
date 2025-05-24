// 95. Unique Binary Search Trees II

use std::cell::RefCell;
use std::rc::Rc;

type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    val: i32,
    left: Tree,
    right: Tree,
}

fn new_tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

mod backtrack;
pub use backtrack::generate_trees;

#[cfg(test)]
mod tests;
