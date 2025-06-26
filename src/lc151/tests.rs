use super::reverse_words;

#[test]
fn t1() {
    let tests = [
        ("the sky is blue", "blue is sky the"),
        ("  hello world  ", "world hello"),
        ("a good   example", "example good a"),
    ];

    for (i, test) in tests.iter().enumerate() {
        assert_eq!(reverse_words(test.0.to_string()), test.1, "{}", i);
    }
}
