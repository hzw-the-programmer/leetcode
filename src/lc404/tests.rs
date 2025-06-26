pub use super::sum_of_left_leaves;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [(btree![3, 9, 20, null, null, 15, 7], 24), (btree![1], 0)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(sum_of_left_leaves(test.0.clone()), test.1, "{}", i);
    }
}
