// time : O(n^2)
// space: O(n)
pub fn verify_tree_order(postorder: Vec<i32>) -> bool {
    recursive(&postorder)
}

fn recursive(postorder: &[i32]) -> bool {
    let len = postorder.len();
    if len < 3 {
        return true;
    }

    let root = postorder[len - 1];
    let mut right_begin = 0;
    while right_begin < len - 1 && postorder[right_begin] < root {
        right_begin += 1;
    }

    for i in right_begin..len - 1 {
        if postorder[i] < root {
            return false;
        }
    }

    recursive(&postorder[..right_begin]) && recursive(&postorder[right_begin..len - 1])
}
