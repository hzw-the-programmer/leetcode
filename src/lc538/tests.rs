pub use super::convert_bst;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (
            btree![
                4, 1, 6, 0, 2, 5, 7, null, null, null, 3, null, null, null, 8
            ],
            btree![
                30, 36, 21, 36, 35, 26, 15, null, null, null, 33, null, null, null, 8
            ],
        ),
        (btree![0, null, 1], btree![1, null, 1]),
        (btree![1, 0, 2], btree![3, 3, 2]),
        (btree![3, 2, 4, 1], btree![7, 9, 4, 10]),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(convert_bst(test.0.clone()), test.1, "{}", i);
    }
}
