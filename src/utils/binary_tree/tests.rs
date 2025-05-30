use super::{build, new_tree as new};

#[test]
fn test_option_array_1() {
    assert_eq!(
        option_array!([3, 9, 20, null, null, 15, 7]),
        [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn test_option_array_2() {
    let res: [Option<i32>; 0] = option_array!([]);
    assert_eq!(res, []);
}

#[test]
fn test_option_array_3() {
    // assert_eq!(
    //     some_array!([3, -9, 20, null, null, 15, 7]),
    //     [Some(3), Some(-9), Some(20), None, None, Some(15), Some(7)]
    // );
    // assert_eq!(some_array!([1, null]), [Some(1), None]);
    // assert_eq!(some_array!([-1, null]), [Some(-1), None]);
    assert_eq!(
        option_array!([3, -9, 20, null, null, 15, 7]),
        [Some(3), Some(-9), Some(20), None, None, Some(15), Some(7)]
    );
}

#[test]
fn test_binary_tree() {
    let wanted = new(
        3,
        new(9, None, None),
        new(20, new(15, None, None), new(7, None, None)),
    );
    // assert_eq!(binary_tree!([3, 9, 20, null, null, 15, 7]), wanted);
    // assert_eq!(binary_tree!(["3", "9", "20", "null", "null", "15", "7"]), wanted);
    assert_eq!(build!([3, 9, 20, null, null, 15, 7]), wanted);
}
