use std::ptr;

#[derive(Clone, PartialEq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn from_slice(vals: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail: *mut ListNode = ptr::null_mut();
    for &val in vals {
        let mut node = Box::new(ListNode::new(val));
        let raw = &mut *node as _;
        if tail.is_null() {
            head = Some(node);
        } else {
            unsafe { (*tail).next = Some(node) };
        }
        tail = raw;
    }
    head
}

#[macro_export]
macro_rules! list {
    [$($input:tt)*] => {
        $crate::utils::singly_linked_list::from_slice(&[$($input)*][..])
    }
}
pub use list;
