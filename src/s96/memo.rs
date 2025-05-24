pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut memo = vec![vec![-1; n + 1]; n + 1];
    num_trees_recursive(1, n, &mut memo)
}

fn num_trees_recursive(s: usize, e: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
    if s > e {
        return 1;
    }
    if memo[s][e] != -1 {
        return memo[s][e];
    }

    let mut sum = 0;

    for i in s..=e {
        let l = num_trees_recursive(s, i - 1, memo);
        let r = num_trees_recursive(i + 1, e, memo);
        sum += l * r;
    }

    memo[s][e] = sum;
    memo[s][e]
}
