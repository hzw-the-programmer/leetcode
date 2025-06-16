use super::BSTIterator;
use crate::utils::binary_tree::btree;

#[test]
fn test() {
    let mut iter = BSTIterator::new(btree![7, 3, 15, null, null, 9, 20]);
    assert_eq!(iter.next(), 3);
    assert_eq!(iter.next(), 7);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 9);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 15);
    assert!(iter.has_next());
    assert_eq!(iter.next(), 20);
    assert!(!iter.has_next());
}
