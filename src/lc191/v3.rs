pub fn hamming_weight(mut n: i32) -> i32 {
    let mut count = 0;
    while n != 0 {
        count += 1;
        println!("{:b}, {:032b}, {}", n, n.wrapping_sub(1), n);
        n = n & n.wrapping_sub(1);
    }
    count
}
