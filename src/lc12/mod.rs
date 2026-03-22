// 12. Integer to Roman

pub fn int_to_roman(num: i32) -> String {
    let num = num as usize;
    let thousands = ["", "M", "MM", "MMM"];
    let hundreds = ["", "C", "CC", "CC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    format!(
        "{}{}{}{}",
        thousands[num / 1000],
        hundreds[num % 1000 / 100],
        tens[num % 100 / 10],
        ones[num % 10]
    )
}

#[cfg(test)]
mod tests;
