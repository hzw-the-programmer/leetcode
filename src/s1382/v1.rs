use crate::utils::binary_tree::*;

pub fn balance_bst(root: Tree) -> Tree {
    let mut nums = vec![];
    let mut current = root;
    while let Some(node) = current {
        if let Some(left) = node.borrow().left.clone() {
            let mut rightmost = left.clone();
            while let Some(right) = {
                let temp = rightmost.borrow().right.clone();
                temp
            } {
                if std::rc::Rc::ptr_eq(&right, &node) {
                    break;
                }
                rightmost = right;
            }
            if rightmost.borrow().right.is_some() {
                nums.push(node.borrow().val);
                rightmost.borrow_mut().right = None;
                current = node.borrow().right.clone();
            } else {
                rightmost.borrow_mut().right = Some(node.clone());
                current = Some(left);
            }
        } else {
            nums.push(node.borrow().val);
            current = node.borrow().right.clone();
        }
    }

    recursive(&nums)
}

fn recursive(nums: &[i32]) -> Tree {
    if nums.is_empty() {
        None
    } else {
        let mid = nums.len() / 2;
        new(
            nums[mid],
            recursive(&nums[..mid]),
            recursive(&nums[mid + 1..]),
        )
    }
}
