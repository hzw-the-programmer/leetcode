// mod non_null;
// pub use non_null::{IntoIter, Iter, IterMut, LinkedList, Node};

mod non_null_dummy;
pub use non_null_dummy::{IntoIter, Iter, IterMut, LinkedList, Node};

#[cfg(test)]
mod tests;
