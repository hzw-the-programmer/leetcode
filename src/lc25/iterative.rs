use crate::utils::singly_linked_list::ListNode;

pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let k = k as usize;
    let mut stack = Vec::with_capacity(k);
    let mut current = head;
    let mut res = None;
    let mut cur = &mut res;

    while let Some(mut node) = current {
        current = node.next.take();
        stack.push(node);
        if stack.len() == k {
            while let Some(node) = stack.pop() {
                (*cur) = Some(node);
                cur = &mut (*cur).as_mut().unwrap().next;
            }
        }
    }

    for node in stack {
        (*cur) = Some(node);
        cur = &mut (*cur).as_mut().unwrap().next;
    }

    res
}
