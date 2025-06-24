pub use super::is_cousins;
pub use crate::utils::binary_tree::btree;

#[test]
fn t1() {
    let tests = [
        (btree![1, 2, 3, 4], 4, 3, false),
        (btree![1, 2, 3, null, 4, null, 5], 5, 4, true),
        (btree![1, 2, 3, null, 4], 2, 3, false),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(is_cousins(test.0.clone(), test.1, test.2), test.3, "{}", i);
    }
}
