use super::*;

#[test]
fn test_generate_trees_1() {
    let expected = vec![new_tree(1, None, None)];
    assert_eq!(generate_trees(1), expected);
}

#[test]
fn test_generate_trees_2() {
    let expected = vec![
        new_tree(1, None, new_tree(2, None, None)),
        new_tree(2, new_tree(1, None, None), None),
    ];
    assert_eq!(generate_trees(2), expected);
}

#[test]
fn test_generate_trees_3() {
    let expected = vec![
        new_tree(1, None, new_tree(2, None, new_tree(3, None, None))),
        new_tree(1, None, new_tree(3, new_tree(2, None, None), None)),
        new_tree(2, new_tree(1, None, None), new_tree(3, None, None)),
        new_tree(3, new_tree(1, None, new_tree(2, None, None)), None),
        new_tree(3, new_tree(2, new_tree(1, None, None), None), None),
    ];
    assert_eq!(generate_trees(3), expected);
}
