// 460. LFU Cache

// mod v1;
// pub use v1::LFUCache;

mod btree_set;
pub use btree_set::LFUCache;

#[cfg(test)]
mod tests;
