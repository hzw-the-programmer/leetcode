// 83. Remove Duplicates from Sorted List

use crate::utils::singly_linked_list::ListNode;

pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head.as_mut();
    while let Some(cur) = current.take() {
        while let Some(next) = cur.next.as_mut() {
            if cur.val == next.val {
                cur.next = next.next.take();
            } else {
                break;
            }
        }
        current = cur.next.as_mut();
    }
    head
}

// pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut current = head.as_mut();
//     while let Some(cur) = current.take() {
//         if let Some(next) = cur.next.as_mut() {
//             if cur.val == next.val {
//                 cur.next = next.next.take();
//                 current = Some(cur);
//             } else {
//                 current = cur.next.as_mut();
//             }
//         }
//     }
//     head
// }

#[cfg(test)]
mod tests;
