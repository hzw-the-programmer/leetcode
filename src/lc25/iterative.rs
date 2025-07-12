use crate::utils::singly_linked_list::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut count = 0;
    let mut cur = head.as_ref();
    while let Some(node) = cur {
        count += 1;
        cur = node.next.as_ref();
    }

    let mut res = None;
    let mut resp = &mut res;

    let mut current = head;
    let mut prev = None;
    for _ in 0..count / k {
        for _ in 0..k {
            let mut node = current.take().unwrap();
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        *resp = prev.take();
        let mut h = (*resp).as_mut().unwrap();
        while h.next.is_some() {
            h = h.next.as_mut().unwrap();
        }
        resp = &mut h.next;
    }

    *resp = current;

    res
}

// pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//     let mut current = head;
//     let mut prev = None;
//     let mut count = 0;

//     while let Some(mut node) = current {
//         current = node.next.take();
//         node.next = prev;
//         prev = Some(node);
//         count += 1;
//     }

//     current = prev.take();
//     let mut remain = count % k;
//     while remain != 0 {
//         let mut node = current.unwrap();
//         current = node.next.take();
//         node.next = prev;
//         prev = Some(node);
//         remain -= 1;
//     }

//     let mut cur = current.as_mut();
//     let mut i = 0;
//     while let Some(node) = cur {
//         i += 1;
//         if i == k {
//             let t = node.next.take();
//             node.next = prev.take();
//             prev = current.take();

//             current = t;
//             cur = current.as_mut();
//             i = 0;
//         } else {
//             cur = node.next.as_mut();
//         }
//     }
//     prev
// }

// pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
//     let k = k as usize;
//     let mut stack = Vec::with_capacity(k);
//     let mut current = head;
//     let mut res = None;
//     let mut cur = &mut res;

//     while let Some(mut node) = current {
//         current = node.next.take();
//         stack.push(node);
//         if stack.len() == k {
//             while let Some(node) = stack.pop() {
//                 (*cur) = Some(node);
//                 cur = &mut (*cur).as_mut().unwrap().next;
//             }
//         }
//     }

//     for node in stack {
//         (*cur) = Some(node);
//         cur = &mut (*cur).as_mut().unwrap().next;
//     }

//     res
// }
