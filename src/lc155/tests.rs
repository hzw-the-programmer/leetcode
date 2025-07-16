use super::MinStack;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [(
        vec![
            "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
        ],
        vec_2d![[], [-2], [0], [-3], [], [], [], []],
        option_array![null, null, null, null, -3, null, 0, -2],
    )];

    for (i, test) in tests.iter().enumerate() {
        driver(&test.0, &test.1, &test.2, i);
    }
}

fn driver(ops: &[&str], params: &[Vec<i32>], results: &[Option<i32>], i: usize) {
    let mut stack = MinStack::new();
    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut stack, i, j);
    }
}

fn test_op(op: &str, param: &[i32], result: Option<i32>, stack: &mut MinStack, i: usize, j: usize) {
    match op {
        "MinStack" => {}
        "push" => stack.push(param[0]),
        "getMin" => assert_eq!(Some(stack.get_min()), result, "{i}.{j}"),
        "pop" => stack.pop(),
        "top" => assert_eq!(Some(stack.top()), result, "{i}.{j}"),
        _ => todo!(),
    }
}
