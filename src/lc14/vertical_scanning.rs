pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    for i in 0..strs[0].len() {
        let c = strs[0].as_bytes()[i];
        for s in &strs[1..] {
            if i == s.len() || s.as_bytes()[i] != c {
                return strs[0][0..i].to_string();
            }
        }
    }

    strs[0].clone()
}
