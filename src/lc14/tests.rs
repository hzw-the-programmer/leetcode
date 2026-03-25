use super::longest_common_prefix;

#[test]
fn t1() {
    let tests = [
        (vec!["flower", "flow", "flight"], "fl"),
        (vec!["dog", "racecar", "car"], ""),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            longest_common_prefix(test.0.iter().map(|s| s.to_string()).collect()),
            test.1,
            "{i}"
        );
    }
}
