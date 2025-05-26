
pub fn valid_utf8(data: Vec<i32>) -> bool {
    let mut i = 0;

    while i < data.len() {
        let first_byte = data[i] as u8;

        let cnt = if first_byte & 0x80 == 0x00 {
            1
        } else if first_byte & 0xe0 == 0xc0 {
            2
        } else if first_byte & 0xf0 == 0xe0 {
            3
        } else if first_byte & 0xf8 == 0xf0 {
            4
        } else {
            return false;
        };

        if i + cnt > data.len() {
            return false;
        }

        for j in 1..cnt {
            if data[i + j] as u8 & 0xc0 != 0x80 {
                return false;
            }
        }

        i += cnt;
    }

    true
}
