use super::jump;

#[test]
fn t1() {
    let tests = [(vec![2, 3, 1, 1, 4], 2), (vec![2, 3, 0, 1, 4], 2)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(jump(test.0.clone()), test.1, "{}", i);
    }
}
