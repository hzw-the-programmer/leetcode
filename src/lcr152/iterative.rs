// time : O(n)
// space: O(n)
pub fn verify_tree_order(postorder: Vec<i32>) -> bool {
    let mut root = i32::MAX;
    let mut stack = Vec::with_capacity(postorder.len());
    for &n in postorder.iter().rev() {
        if n > root {
            return false;
        }
        while let Some(&top) = stack.last() {
            if n > top {
                break;
            }
            root = stack.pop().unwrap();
        }
        stack.push(n);
    }
    true
}
