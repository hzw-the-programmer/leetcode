use super::verify_tree_order;

#[test]
fn test() {
    assert!(!verify_tree_order(vec![4, 9, 6, 5, 8]));
    assert!(verify_tree_order(vec![4, 6, 5, 9, 8]));
}
