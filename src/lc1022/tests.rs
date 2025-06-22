use super::sum_root_to_leaf;
use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [(btree![1, 0, 1, 0, 1, 0, 1], 22), (btree![0], 0)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(sum_root_to_leaf(test.0.clone()), test.1, "{}", i);
    }
}
