// pub fn hamming_weight(n: i32) -> i32 {
//     let mut count = 0;
//     let mut flag = 1;
//     while flag != 0 {
//         if n & flag == flag {
//             count += 1;
//         }
//         flag <<= 1;
//     }
//     count
// }

pub fn hamming_weight(n: i32) -> i32 {
    let mut count = 0;
    for i in 0..32 {
        if n & 1 << i != 0 {
            count += 1;
        }
    }
    count
}
