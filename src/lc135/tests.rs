use super::candy;

#[test]
fn t1() {
    let tests = [(vec![1, 0, 2], 5), (vec![1, 2, 2], 4)];
    for (i, test) in tests.iter().enumerate() {
        assert_eq!(candy(test.0.clone()), test.1, "{i}");
    }
}
