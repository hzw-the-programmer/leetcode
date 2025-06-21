pub fn is_valid_serialization(preorder: String) -> bool {
    let mut slots = 1;
    for s in preorder.split(',') {
        slots -= 1;
        if slots < 0 {
            return false;
        }
        match s {
            "#" => {}
            _ => slots += 2,
        }
    }
    slots == 0
}
