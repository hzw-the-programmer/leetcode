// mod non_null;
// pub use non_null::{Iter, LinkedList, Node};

mod non_null_dummy;
pub use non_null_dummy::{Iter, LinkedList, Node};

#[cfg(test)]
mod tests;
