use super::max_profit;

#[test]
fn t1() {
    let tests = [(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(max_profit(test.0.clone()), test.1, "{}", i);
    }
}
