use super::KthLargest;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [
        (
            vec!["KthLargest", "add", "add", "add", "add", "add"],
            // vec_2d![[3, [4, 5, 8, 2]], [3], [5], [10], [9], [4]],
            vec_2d![[3, 4, 5, 8, 2], [3], [5], [10], [9], [4]],
            option_array![null, 4, 5, 5, 8, 8],
        ),
        (
            vec!["KthLargest", "add", "add", "add", "add"],
            // vec_2d![[4, [7, 7, 7, 7, 8, 3]], [2], [10], [9], [9]],
            vec_2d![[4, 7, 7, 7, 7, 8, 3], [2], [10], [9], [9]],
            option_array![null, 7, 7, 7, 8],
        ),
    ];

    for (i, test) in tests.iter().enumerate() {
        drive(&test.0, &test.1, &test.2, i);
    }
}

fn drive(ops: &[&str], params: &[Vec<i32>], results: &[Option<i32>], i: usize) {
    let mut kth = KthLargest::new(params[0][0], params[0][1..].to_vec());
    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        match *op {
            "KthLargest" => {}
            "add" => assert_eq!(Some(kth.add(param[0])), *result, "{i}.{j}"),
            _ => todo!(),
        }
    }
}
