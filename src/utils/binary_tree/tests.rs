#[test]
fn test_1() {
    assert_eq!(
        option_array!([3, 9, 20, null, null, 15, 7]),
        [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
}
