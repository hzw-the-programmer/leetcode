use super::WordDictionary;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [
        (
            vec![
                "WordDictionary",
                "addWord",
                "addWord",
                "addWord",
                "search",
                "search",
                "search",
                "search",
            ],
            vec_2d![
                [],
                ["bad"],
                ["dad"],
                ["mad"],
                ["pad"],
                ["bad"],
                [".ad"],
                ["b.."]
            ],
            option_array![null, null, null, null, false, true, true, true],
        ),
        (
            vec![
                "WordDictionary",
                "addWord",
                "addWord",
                "search",
                "search",
                "search",
                "search",
                "search",
                "search",
            ],
            vec_2d![
                [],
                ["a"],
                ["a"],
                ["."],
                ["a"],
                ["aa"],
                ["a"],
                [".a"],
                ["a."]
            ],
            option_array![null, null, null, true, true, false, true, false, false],
        ),
    ];

    for (i, test) in tests.iter().enumerate() {
        driver(&test.0, &test.1, &test.2, i);
    }
}

fn driver(ops: &[&str], params: &[Vec<&str>], results: &[Option<bool>], i: usize) {
    let mut dic = WordDictionary::new();

    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut dic, i, j);
    }
}

fn test_op(
    op: &str,
    param: &[&str],
    result: Option<bool>,
    dic: &mut WordDictionary,
    i: usize,
    j: usize,
) {
    match op {
        "WordDictionary" => {}
        "addWord" => dic.add_word(param[0].to_string()),
        "search" => assert_eq!(Some(dic.search(param[0].to_string())), result, "{i}.{j}"),
        _ => todo!(),
    }
}
