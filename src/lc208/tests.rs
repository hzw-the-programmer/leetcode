use super::Trie;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [(
        vec![
            "Trie",
            "insert",
            "search",
            "search",
            "startsWith",
            "insert",
            "search",
        ],
        vec_2d![[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]],
        option_array![null, null, true, false, true, null, true],
    )];

    for (i, test) in tests.iter().enumerate() {
        test_driver(&test.0, &test.1, &test.2, i);
    }
}

fn test_driver(ops: &[&str], params: &[Vec<&str>], results: &[Option<bool>], i: usize) {
    let mut trie = Trie::new();
    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut trie, i, j);
    }
}

fn test_op(op: &str, param: &[&str], result: Option<bool>, trie: &mut Trie, i: usize, j: usize) {
    match op {
        "Trie" => {}
        "insert" => trie.insert(param[0].to_string()),
        "search" => assert_eq!(Some(trie.search(param[0].to_string())), result, "{i}.{j}"),
        "startsWith" => assert_eq!(
            Some(trie.starts_with(param[0].to_string())),
            result,
            "{i}.{j}"
        ),
        _ => todo!(),
    }
}
