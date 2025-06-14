pub fn hamming_weight(n: i32) -> i32 {
    let mut count = 0;
    let mut flag = 1;
    while flag != 0 {
        if n & flag == flag {
            count += 1;
        }
        flag <<= 1;
    }
    count
}
