pub fn verify_tree_order(postorder: Vec<i32>) -> bool {
    recursive(&postorder)
}

fn recursive(postorder: &[i32]) -> bool {
    let len = postorder.len();
    if len < 3 {
        return true;
    }

    let root = postorder[len - 1];
    let mut left_end = len - 1;
    for i in 0..len - 1 {
        if postorder[i] > root {
            left_end = i;
            break;
        }
    }

    for i in left_end..len - 1 {
        if postorder[i] < root {
            return false;
        }
    }

    recursive(&postorder[..left_end]) && recursive(&postorder[left_end..len - 1])
}
