use super::delete_node;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (
            btree![5, 3, 6, 2, 4, null, 7],
            3,
            btree![5, 4, 6, 2, null, null, 7],
        ),
        (
            btree![5, 3, 6, 2, 4, null, 7],
            0,
            btree![5, 3, 6, 2, 4, null, 7],
        ),
        (btree![], 0, btree![]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(delete_node(test.0.clone(), test.1), test.2, "{}", i);
    }
}
