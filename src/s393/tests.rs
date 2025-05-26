use super::*;

#[test]
fn test_valid_utf8() {
    assert!(valid_utf8(vec![197, 130, 1]));
    assert!(!valid_utf8(vec![235, 140, 4]));
}
