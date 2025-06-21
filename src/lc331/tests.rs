use super::is_valid_serialization;

#[test]
fn t1() {
    let tests = [
        ("9,3,4,#,#,1,#,#,2,#,6,#,#", true),
        ("1,#", false),
        ("9,#,#,1", false),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(
            is_valid_serialization(test.0.to_string()),
            test.1,
            "{} failed.",
            i
        );
    }
}
