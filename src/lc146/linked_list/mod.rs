// mod non_null;
// pub use non_null::{Iter, LinkedList, Node};

mod non_null_dummy;
pub use non_null_dummy::{Iter, LinkedList, Node};

// mod rc_dummy;
// pub use rc_dummy::{Iter, LinkedList, NodePtr};

#[cfg(test)]
mod tests;
