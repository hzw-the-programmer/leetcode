use crate::utils::binary_tree::*;

pub fn flatten(root: &mut Tree) {
    let mut cur = root.clone();

    while let Some(node) = cur {
        let mut borrowed = node.borrow_mut();
        if let Some(left) = borrowed.left.take() {
            let mut rightmost = left.clone();
            while let Some(right) = {
                let temp = rightmost.borrow().right.clone();
                temp
            } {
                rightmost = right;
            }
            rightmost.borrow_mut().right = borrowed.right.clone();
            borrowed.right = Some(left);
        }
        cur = borrowed.right.clone();
    }
}

// pub fn flatten(root: &mut Tree) {
//     let mut root = root.clone();
//     while let Some(node) = root {
//         if let Some(left) = {
//             let temp = node.borrow().left.clone();
//             temp
//         } {
//             let mut rightmost = left.clone();
//             while let Some(right) = {
//                 let temp = rightmost.borrow().right.clone();
//                 temp
//             } {
//                 rightmost = right;
//             }
//             rightmost.borrow_mut().right = node.borrow().right.clone();

//             node.borrow_mut().right = Some(left);
//             node.borrow_mut().left = None;
//         }

//         root = node.borrow().right.clone();
//     }
// }
