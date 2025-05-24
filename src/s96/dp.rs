pub fn num_trees(n: i32) -> i32 {
    let n = n as usize;
    let mut g = vec![0; n + 1];
    g[0] = 1;
    g[1] = 1;
    for i in 2..=n {
        for j in 1..=i {
            g[i] += g[j - 1] * g[i - j];
        }
    }
    g[n]
}
