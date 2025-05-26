use super::*;

#[test]
fn test_my_atoi() {
    assert_eq!(42, my_atoi("42".to_string()));
    assert_eq!(-42, my_atoi(" -042".to_string()));
    assert_eq!(1337, my_atoi("1337c0d3".to_string()));
    assert_eq!(0, my_atoi("0-1".to_string()));
    assert_eq!(0, my_atoi("words and 987".to_string()));
    assert_eq!(-2147483648, my_atoi("-91283472332".to_string()));
}
