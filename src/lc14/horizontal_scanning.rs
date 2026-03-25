pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.iter().skip(1).fold(
        strs.get(0).unwrap_or(&String::new()).clone(),
        |prefix, s| {
            prefix
                .chars()
                .zip(s.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        },
    )
}

// pub fn longest_common_prefix(strs: Vec<String>) -> String {
//     if strs.is_empty() {
//         return String::new();
//     }

//     let mut prefix = strs[0].clone();
//     for s in &strs[1..] {
//         while !s.starts_with(&prefix) {
//             prefix.pop();
//             if prefix.is_empty() {
//                 return String::new();
//             }
//         }
//     }
//     prefix
// }

// pub fn longest_common_prefix(strs: Vec<String>) -> String {
//     if strs.is_empty() {
//         return "".to_string();
//     }

//     let mut prefix = strs[0].as_str();
//     for i in 1..strs.len() {
//         prefix = longest_common_prefix_2(prefix, &strs[i]);
//         if prefix.len() == 0 {
//             break;
//         }
//     }
//     prefix.to_string()
// }

// fn longest_common_prefix_2<'a>(str1: &'a str, str2: &'a str) -> &'a str {
//     let (b1, b2) = (str1.as_bytes(), str2.as_bytes());
//     let n = b1.len().min(b2.len());
//     let mut i = 0;
//     while i < n && b1[i] == b2[i] {
//         i += 1;
//     }
//     str::from_utf8(&b1[..i]).unwrap()
// }
