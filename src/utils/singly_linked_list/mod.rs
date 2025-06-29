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
    let (mut head, mut tail) = (None, None);

    for &val in vals {
        let node = Box::new(ListNode::new(val));
        if tail.is_none() {
            head = Some(node);
            tail = head.as_mut();
        } else {
            let t = tail.unwrap();
            t.next = Some(node);
            tail = t.next.as_mut();
        }
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
