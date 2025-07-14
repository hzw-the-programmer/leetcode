// 2. Add Two Numbers

use crate::utils::singly_linked_list::ListNode;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut curry = 0;
    let mut res = None;
    let mut resp = &mut res;

    let (mut list1, mut list2) = (l1, l2);
    loop {
        match (list1, list2) {
            (None, None) => break,
            (Some(mut node), None) | (None, Some(mut node)) => {
                node.val += curry;
                curry = node.val / 10;
                node.val %= 10;
                *resp = Some(node);
                resp = &mut (*resp).as_mut().unwrap().next;

                if curry == 0 {
                    break;
                } else {
                    (list1, list2) = ((*resp).take(), None);
                }
            }
            (Some(mut node1), Some(mut node2)) => {
                node1.val += node2.val + curry;
                curry = node1.val / 10;
                node1.val %= 10;
                *resp = Some(node1);
                resp = &mut (*resp).as_mut().unwrap().next;

                (list1, list2) = ((*resp).take(), node2.next.take());
            }
        }
    }
    if curry > 0 {
        *resp = Some(Box::new(ListNode::new(curry)));
    }

    res
}

#[cfg(test)]
mod tests;
