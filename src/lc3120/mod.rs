/// 3120. 统计特殊字母的数量 I
/// A: 0x41 0b0100_0001
/// a: 0x61 0b0110_0001
/// B: 0x42 0b0100_0002
/// b: 0x62 0b0110_0002
/// Z: 0x5A 0b0101_1010
/// z: 0x7A 0b0111_1010

/// cargo test lc3120 -- --show-output

pub fn number_of_special_chars(word: String) -> i32 {
    let mut mask = [0u32; 2];
    for c in word.chars() {
        // println!("{:08b}", c as u32);
        mask[((c as usize) & 0x20) >> 5] |= 1 << ((c as u32) & 0x1f);
        // println!(
        //     "{}: {:032b}",
        //     ((c as usize) & 0x20) >> 5,
        //     mask[((c as usize) & 0x20) >> 5]
        // );
    }
    (mask[0] & mask[1]).count_ones() as _
}

#[cfg(test)]
mod tests {
    use super::number_of_special_chars;

    #[test]
    fn f1() {
        assert_eq!(number_of_special_chars("aaAbcBC".into()), 3);
        assert_eq!(number_of_special_chars("abc".into()), 0);
        assert_eq!(number_of_special_chars("abBCab".into()), 1);
    }
}
