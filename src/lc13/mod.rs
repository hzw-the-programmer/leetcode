// 13. Roman to Integer

pub fn roman_to_int(s: String) -> i32 {
    let s = s.as_bytes();
    let mut total = 0;
    let mut pre = 0;

    for &c in s.iter().rev() {
        let v = match c {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => todo!(),
        };

        if v < pre {
            total -= v;
        } else {
            total += v;
        }

        pre = v;
    }

    total
}

#[cfg(test)]
mod tests;
