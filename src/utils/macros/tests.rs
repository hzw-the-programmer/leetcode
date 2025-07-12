#[test]
fn test_option_array_1() {
    assert_eq!(
        option_array![3, 9, 20, null, null, 15, 7],
        [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn test_option_array_2() {
    let res: Vec<Option<i32>> = option_array![];
    assert_eq!(res, []);
}

#[test]
fn test_option_array_3() {
    assert_eq!(
        option_array![3, -9, 20, null, null, 15, 7],
        [Some(3), Some(-9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn t1() {
    assert_eq!(option_array![1, -2, 3], [Some(1), Some(-2), Some(3)]);
    assert_eq!(
        option_array![1, null, -2, 3],
        [Some(1), None, Some(-2), Some(3)]
    );
    assert_eq!(
        option_array![1, null, -2, 3, null],
        [Some(1), None, Some(-2), Some(3), None]
    );
    assert_eq!(
        option_array![[1], null, [-2], [3]],
        [Some(vec![1]), None, Some(vec![-2]), Some(vec![3])]
    );
}
