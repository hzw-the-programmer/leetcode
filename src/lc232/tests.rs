use super::MyQueue;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [(
        vec!["MyQueue", "push", "push", "peek", "pop", "empty"],
        vec_2d![[], [1], [2], [], [], []],
        // option_array![null, null, null, 1, 1, false],
        option_array![null, null, null, 1, 1, 0],
    )];

    for (i, test) in tests.iter().enumerate() {
        drive(&test.0, &test.1, &test.2, i);
    }
}

fn drive(ops: &[&str], params: &[Vec<i32>], results: &[Option<i32>], i: usize) {
    let mut queue = MyQueue::new();
    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut queue, i, j);
    }
}

fn test_op(op: &str, param: &[i32], result: Option<i32>, queue: &mut MyQueue, i: usize, j: usize) {
    match op {
        "MyQueue" => {}
        "push" => queue.push(param[0]),
        "peek" => assert_eq!(Some(queue.peek()), result, "{i}.{j}"),
        "pop" => assert_eq!(Some(queue.pop()), result, "{i}.{j}"),
        // "empty" => assert_eq!(Some(queue.empty()), result, "{i}.{j}"),
        "empty" => assert_eq!(Some(queue.empty() as i32), result, "{i}.{j}"),
        _ => todo!(),
    }
}
