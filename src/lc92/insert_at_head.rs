use crate::utils::singly_linked_list::ListNode;

pub fn reverse_between(
    mut head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    let mut tail = &mut head;
    for _ in 0..left - 1 {
        tail = &mut (*tail).as_mut().unwrap().next;
    }

    let node: *mut ListNode = (*tail).as_deref_mut().unwrap();

    for _ in 0..right - left {
        unsafe {
            if let Some(mut next) = (*node).next.take() {
                (*node).next = next.next.take();
                next.next = (*tail).take();
                *tail = Some(next);
            }
        }
    }

    head
}
