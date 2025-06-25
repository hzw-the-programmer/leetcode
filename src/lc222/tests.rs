pub use super::count_nodes;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [(btree![1, 2, 3, 4, 5, 6], 6), (btree![], 0), (btree![1], 1)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(count_nodes(test.0.clone()), test.1, "{}", i);
    }
}
