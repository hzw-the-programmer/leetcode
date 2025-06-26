pub fn reverse_words(s: String) -> String {
    let mut s = s
        .trim()
        .split_whitespace()
        .rev()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc.push(' ');
            acc
        });
    if s.len() > 0 {
        s.truncate(s.len() - 1);
    }
    s
}
