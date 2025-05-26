use super::*;

#[test]
fn test_is_number() {
    assert!(is_number("2".to_string()));
    assert!(is_number("0089".to_string()));
    assert!(is_number("-0.1".to_string()));
    assert!(is_number("+3.14".to_string()));
    assert!(is_number("4.".to_string()));
    assert!(is_number("-.9".to_string()));
    assert!(is_number("2e10".to_string()));
    assert!(is_number("-90E3".to_string()));
    assert!(is_number("3e+7".to_string()));
    assert!(is_number("+6e-1".to_string()));
    assert!(is_number("53.5e93".to_string()));
    assert!(is_number("-123.456e789".to_string()));

    assert!(is_number("0".to_string()));
}

#[test]
fn test_is_not_number() {
    assert!(!is_number("abc".to_string()));
    assert!(!is_number("1a".to_string()));
    assert!(!is_number("1e".to_string()));
    assert!(!is_number("e3".to_string()));
    assert!(!is_number("99e2.5".to_string()));
    assert!(!is_number("--6".to_string()));
    assert!(!is_number("-+3".to_string()));
    assert!(!is_number("95a54e53".to_string()));

    assert!(!is_number("e".to_string()));
    assert!(!is_number(".".to_string()));
}
