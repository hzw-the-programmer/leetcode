// 297. Serialize and Deserialize Binary Tree

pub mod inorder;
pub mod levelorder;
pub mod preorder;

pub fn parse_i32(data: &mut &str) -> Result<i32, u8> {
    let (mut n, mut sign, mut len) = (0, 1, 0);

    let mut bytes = data.as_bytes();

    if bytes.is_empty() {
        return Err(3);
    }

    if bytes[0] == b'-' {
        sign = -1;
        len += 1;
        bytes = &bytes[1..];
    }

    if bytes.is_empty() || to_digit(bytes[0]).is_none() {
        return Err(4);
    }

    for &b in bytes {
        if let Some(i) = to_digit(b) {
            n = n * 10 + i;
            len += 1;
        } else {
            break;
        }
    }

    *data = &data[len..];

    Ok(n * sign)
}

pub fn to_digit(b: u8) -> Option<i32> {
    let i = b as i32 - b'0' as i32;
    if i >= 0 && i <= 10 { Some(i) } else { None }
}
