use super::MedianFinder;
use crate::utils::macros::{option_array, vec_2d};

#[test]
fn t1() {
    let tests = [
        (
            vec![
                "MedianFinder",
                "addNum",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
            ],
            vec_2d![[], [1], [2], [], [3], []],
            option_array![null, null, null, 1.5, null, 2.0],
        ),
        (
            vec!["MedianFinder", "addNum", "findMedian"],
            vec_2d![[], [1], []],
            option_array![null, null, 1.0],
        ),
        (
            vec![
                "MedianFinder",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
            ],
            vec_2d![[], [-1], [], [-2], [], [-3], [], [-4], [], [-5], []],
            option_array![
                null, null, -1.0, null, -1.5, null, -2.0, null, -2.5, null, -3.0
            ],
        ),
        (
            vec![
                "MedianFinder",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
                "addNum",
                "findMedian",
            ],
            vec_2d![
                [],
                [6],
                [],
                [10],
                [],
                [2],
                [],
                [6],
                [],
                [5],
                [],
                [0],
                [],
                [6],
                [],
                [3],
                [],
                [1],
                [],
                [0],
                [],
                [0],
                []
            ],
            option_array![
                null, null, 6.0, null, 8.0, null, 6.0, null, 6.0, null, 6.0, null, 5.5, null, 6.0,
                null, 5.5, null, 5.0, null, 4.0, null, 3.0
            ],
        ),
    ];

    for (i, test) in tests.iter().enumerate() {
        drive(&test.0, &test.1, &test.2, i);
    }
}

fn drive(ops: &[&str], params: &[Vec<i32>], results: &[Option<f64>], i: usize) {
    let mut finder = MedianFinder::new();

    for (j, ((op, param), result)) in ops.iter().zip(params).zip(results).enumerate() {
        test_op(op, param, *result, &mut finder, i, j);
    }
}

fn test_op(
    op: &str,
    param: &[i32],
    result: Option<f64>,
    finder: &mut MedianFinder,
    i: usize,
    j: usize,
) {
    match op {
        "MedianFinder" => {}
        "addNum" => finder.add_num(param[0]),
        "findMedian" => assert_eq!(Some(finder.find_median()), result, "{i}.{j}"),
        _ => todo!(),
    }
}
