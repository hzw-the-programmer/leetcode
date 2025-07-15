use crate::utils::singly_linked_list::ListNode;

pub fn reverse_between(
    mut head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut cur = &mut head;
    for _ in 0..left - 1 {
        cur = &mut (*cur).as_mut().unwrap().next;
    }
    let mut left_node = (*cur).take();

    // if left_node.is_none() {
    //     return head;
    // }

    let left_node_next: *mut Option<Box<ListNode>> = &mut left_node.as_mut().unwrap().next;

    let mut right_node = left_node;
    let mut prev = None;
    for _ in 0..right - left + 1 {
        let mut node = right_node.unwrap();
        right_node = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    *cur = prev;
    unsafe {
        *left_node_next = right_node;
    }

    head
}

// pub fn reverse_between(
//     mut head: Option<Box<ListNode>>,
//     left: i32,
//     right: i32,
// ) -> Option<Box<ListNode>> {
//     let mut begin = None;

//     let mut i = 1;
//     let mut cur = &mut head;
//     while let Some(node) = (*cur).take() {
//         if i == left {
//             begin = Some(node);
//             break;
//         } else {
//             (*cur) = Some(node);
//             cur = &mut (*cur).as_mut().unwrap().next;
//             i += 1;
//         }
//     }

//     if begin.is_none() {
//         return head;
//     }

//     let p: *mut Option<Box<ListNode>> = &mut begin.as_mut().unwrap().next;
//     let mut prev = None;
//     while let Some(mut node) = begin {
//         begin = node.next.take();
//         node.next = prev;
//         prev = Some(node);
//         if i == right {
//             break;
//         }
//         i += 1;
//     }

//     (*cur) = prev;
//     unsafe {
//         *p = begin;
//     }

//     head
// }
