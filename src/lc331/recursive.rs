pub fn is_valid_serialization(preorder: String) -> bool {
    let mut iter = preorder.split(',').map(|s| match s {
        "#" => None,
        _ => s.parse().ok(),
    });

    recursive(&mut iter) && iter.next().is_none()
}

fn recursive(iter: &mut impl Iterator<Item = Option<i32>>) -> bool {
    match iter.next() {
        None => false,
        Some(None) => true,
        Some(Some(_)) => recursive(iter) && recursive(iter),
    }
}
