pub fn num_trees(n: i32) -> i32 {
    num_trees_recursive(1, n)
}

fn num_trees_recursive(s: i32, e: i32) -> i32 {
    if s > e {
        return 1;
    }

    let mut sum = 0;

    for i in s..=e {
        let l = num_trees_recursive(s, i - 1);
        let r = num_trees_recursive(i + 1, e);
        sum += l * r;
    }

    sum
}
