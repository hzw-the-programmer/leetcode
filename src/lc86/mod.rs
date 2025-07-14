// 86. Partition List

use crate::utils::singly_linked_list::ListNode;

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let (mut lt, mut ge) = (None, None);
    let (mut ltp, mut gep) = (&mut lt, &mut ge);

    let mut cur = head;
    while let Some(mut node) = cur.take() {
        cur = node.next.take();
        if node.val < x {
            *ltp = Some(node);
            ltp = &mut (*ltp).as_mut().unwrap().next;
        } else {
            *gep = Some(node);
            gep = &mut (*gep).as_mut().unwrap().next;
        }
    }

    (*ltp) = ge;

    lt
}

#[cfg(test)]
mod tests;
