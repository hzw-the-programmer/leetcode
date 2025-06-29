use crate::utils::singly_linked_list::ListNode;

pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut current = &mut dummy;
    while let Some(next) = current.next.as_mut() {
        if next.val == val {
            current.next = next.next.take();
            break;
        }
        current = current.next.as_mut().unwrap();
    }
    dummy.next
}

// pub fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
//     match head {
//         None => None,
//         Some(mut head) => {
//             if val == head.val {
//                 head.next.take()
//             } else {
//                 let mut current = head.as_mut();
//                 loop {
//                     let next = current.next.take();
//                     match next {
//                         None => break,
//                         Some(mut node) => {
//                             if node.val == val {
//                                 current.next = node.next.take();
//                                 break;
//                             } else {
//                                 current.next = Some(node);
//                                 current = current.next.as_mut().unwrap();
//                             }
//                         }
//                     }
//                 }
//                 Some(head)
//             }
//         }
//     }
// }
