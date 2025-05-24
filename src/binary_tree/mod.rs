use std::cell::RefCell;
use std::rc::Rc;

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub fn new_tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
