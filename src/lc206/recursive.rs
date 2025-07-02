use crate::utils::singly_linked_list::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn recursive(head: Option<Box<ListNode>>, pre: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => pre,
            Some(mut node) => {
                let next = node.next.take();
                node.next = pre;
                recursive(next, Some(node))
            }
        }
    }

    recursive(head, None)
}

// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     fn recursive(cur: Box<ListNode>, mut next: Box<ListNode>) -> Option<Box<ListNode>> {
//         match next.next.replace(cur) {
//             None => Some(next),
//             Some(node) => recursive(next, node),
//         }
//     }

//     let mut cur = head?;
//     if let Some(next) = cur.next.take() {
//         recursive(cur, next)
//     } else {
//         Some(cur)
//     }
// }

// pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     fn recursive(mut next: Box<ListNode>, cur: Box<ListNode>) -> Option<Box<ListNode>> {
//         if next.next.is_none() {
//             next.next = Some(cur);
//             Some(next)
//         } else {
//             recursive(next.next.replace(cur).unwrap(), next)
//         }
//     }

//     let mut cur = head?;
//     if let Some(next) = cur.next.take() {
//         recursive(next, cur)
//     } else {
//         Some(cur)
//     }
// }
