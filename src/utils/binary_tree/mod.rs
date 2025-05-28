use std::cell::RefCell;
use std::rc::Rc;

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

pub fn new_tree(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}

macro_rules! option_array {
    // 主规则：匹配数组并处理每个元素
    ([ $($elem:tt),* ]) => {
        [ $(option_array!(@parse $elem)),* ]
    };
    // 内部规则：处理 `null` 转换为 `None`
    (@parse null) => {
        None
    };
    // 内部规则：处理非 `null` 元素，包裹为 `Some`
    (@parse $num:expr) => {
        Some($num)
    }
}

#[cfg(test)]
mod tests;
