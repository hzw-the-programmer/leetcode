use super::*;

#[test]
fn test_num_trees_1() {
    assert_eq!(num_trees(3), 5);
    assert_eq!(num_trees(1), 1);
}

#[test]
fn test_num_trees_2() {
    assert_eq!(num_trees(19), 1767263190);
}
