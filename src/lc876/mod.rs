// 876. Middle of the Linked List

use crate::utils::singly_linked_list::ListNode;

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut fast = head.as_ref();
    let mut slow = fast;

    while fast.is_some() && fast.unwrap().next.is_some() {
        fast = fast.unwrap().next.as_ref().unwrap().next.as_ref();
        slow = slow.unwrap().next.as_ref();
    }

    slow.cloned()
}

// pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut fast = head.as_ref();
//     let mut slow = fast;

//     while let Some(node) = fast {
//         fast = node.next.as_ref();
//         if let Some(node) = fast {
//             fast = node.next.as_ref();
//             slow = slow.unwrap().next.as_ref();
//         }
//     }

//     slow.cloned()
// }

// pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut fast = head.as_ref()?;
//     let mut slow = fast;

//     while let Some(node) = fast.next.as_ref() {
//         fast = node;
//         slow = slow.next.as_ref().unwrap();
//         if let Some(node) = fast.next.as_ref() {
//             fast = node;
//         }
//     }

//     Some(slow.clone())
// }

// pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut fast: *mut ListNode = head.as_deref_mut()?;
//     let mut dummy = ListNode { val: 0, next: head };
//     let mut slow = &mut dummy;

//     unsafe {
//         while let Some(node) = (*fast).next.as_deref_mut() {
//             fast = node;
//             slow = (*slow).next.as_deref_mut().unwrap();
//             if let Some(node) = (*fast).next.as_deref_mut() {
//                 fast = node;
//             }
//         }

//         (*slow).next.take()
//     }
// }

// pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     let mut fast: *mut ListNode = head.as_deref_mut()?;
//     let mut slow = fast;

//     unsafe {
//         if (*fast).next.is_none() {
//             return head;
//         }
//         while let Some(node) = (*fast).next.as_deref_mut() {
//             fast = node;
//             if let Some(node) = (*fast).next.as_deref_mut() {
//                 fast = node;
//                 if (*fast).next.is_some() {
//                     slow = (*slow).next.as_deref_mut().unwrap();
//                 }
//             }
//         }
//         (*slow).next.take()
//     }
// }

#[cfg(test)]
mod tests;
