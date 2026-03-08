use super::can_jump;

#[test]
fn t1() {
    let tests = [(vec![2, 3, 1, 1, 4], true), (vec![3, 2, 1, 0, 4], false)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(can_jump(test.0.clone()), test.1, "{}", i);
    }
}
