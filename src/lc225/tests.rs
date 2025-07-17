use super::MyStack;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [(
        vec!["MyStack", "push", "push", "top", "pop", "empty"],
        vec_2d![[], [1], [2], [], [], []],
        // option_array![null, null, null, 2, 2, false],
        option_array![null, null, null, 2, 2, 0],
    )];

    for (i, test) in tests.iter().enumerate() {
        drive(&test.0, &test.1, &test.2, i);
    }
}

fn drive(ops: &[&str], params: &[Vec<i32>], results: &[Option<i32>], i: usize) {
    let mut stack = MyStack::new();

    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut stack, i, j);
    }
}

fn test_op(op: &str, param: &[i32], result: Option<i32>, stack: &mut MyStack, i: usize, j: usize) {
    match op {
        "MyStack" => {}
        "push" => stack.push(param[0]),
        "top" => assert_eq!(Some(stack.top()), result, "{i}.{j}"),
        "pop" => assert_eq!(Some(stack.pop()), result, "{i}.{j}"),
        // "empty" => assert_eq!(Some(stack.empty()), result, "{i}.{j}"),
        "empty" => assert_eq!(Some(stack.empty() as i32), result, "{i}.{j}"),
        _ => todo!(),
    }
}
