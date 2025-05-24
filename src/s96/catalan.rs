pub fn num_trees(n: i32) -> i32 {
    let n = n as u64;
    let mut c = 1;
    for i in 0..n {
        // c = (2 * (2 * i + 1) / (i + 2)) * c;
        // c *= 2 * (2 * i + 1) / (i + 2);
        c = c * 2 * (2 * i + 1) / (i + 2);
    }
    c as _
}
