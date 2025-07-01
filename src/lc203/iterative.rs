use crate::utils::singly_linked_list::ListNode;

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut cur = head.as_mut()?;
    while cur.val == val {
        head = cur.next.take();
        cur = head.as_mut()?;
    }

    while let Some(next) = cur.next.as_mut() {
        if next.val == val {
            cur.next = next.next.take();
        } else {
            cur = cur.next.as_mut().unwrap();
        }
    }

    head
}

// pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     let mut dummy = ListNode { val: 0, next: head };
//     let mut cur = &mut dummy;
//     while let Some(node) = cur.next.as_mut() {
//         if node.val == val {
//             cur.next = node.next.take();
//         } else {
//             cur = cur.next.as_deref_mut().unwrap();
//         }
//     }
//     dummy.next
// }
