use super::*;
use crate::binary_tree::*;

#[test]
fn test_1() {
    let mut root = new_tree(
        1,
        new_tree(2, new_tree(3, None, None), new_tree(4, None, None)),
        new_tree(5, None, new_tree(6, None, None)),
    );
    let expected = new_tree(
        1,
        None,
        new_tree(
            2,
            None,
            new_tree(
                3,
                None,
                new_tree(4, None, new_tree(5, None, new_tree(6, None, None))),
            ),
        ),
    );
    flatten(&mut root);
    assert_eq!(root, expected);
}
