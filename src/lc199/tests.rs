pub use super::right_side_view;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![1, 2, 3, null, 5, null, 4], vec![1, 3, 4]),
        (btree![1, 2, 3, 4, null, null, null, 5], vec![1, 3, 4, 5]),
        (btree![1, null, 3], vec![1, 3]),
        (btree![], vec![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(right_side_view(test.0.clone()), test.1, "{}", i);
    }
}
