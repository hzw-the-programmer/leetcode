pub fn reverse_words(s: String) -> String {
    let mut s = s
        .split_whitespace()
        .rev()
        .fold(String::new(), |mut acc, s| {
            acc.push_str(s);
            acc.push(' ');
            acc
        });
    s.pop();
    s
}
