use std::ptr;

pub struct ListNode {
    pub(crate) val: i32,
    pub(crate) next: Option<Box<ListNode>>,
}

pub fn new_from(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail: *mut ListNode = ptr::null_mut();
    for &val in arr {
        let mut node = Box::new(ListNode { val, next: None });
        let raw = &mut *node as *mut ListNode;
        if tail.is_null() {
            head = Some(node);
        } else {
            unsafe { (*tail).next = Some(node) };
        }
        tail = raw;
    }
    head
}
