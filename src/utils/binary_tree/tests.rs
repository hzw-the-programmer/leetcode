use super::{btree, new_tree as new};

#[test]
fn test_binary_tree() {
    let wanted = new(
        3,
        new(9, None, None),
        new(20, new(15, None, None), new(7, None, None)),
    );
    // assert_eq!(binary_tree!([3, 9, 20, null, null, 15, 7]), wanted);
    // assert_eq!(binary_tree!(["3", "9", "20", "null", "null", "15", "7"]), wanted);
    assert_eq!(btree![3, 9, 20, null, null, 15, 7], wanted);
}
