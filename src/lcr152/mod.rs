// LCR 152. 验证二叉搜索树的后序遍历序列

// mod recursive;
// pub use recursive::verify_tree_order;

mod iterative;
pub use iterative::verify_tree_order;

#[cfg(test)]
mod tests;
