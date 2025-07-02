use crate::utils::singly_linked_list::ListNode;

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut head = head.as_ref();
    let mut res = 0;
    while let Some(node) = head {
        head = node.next.as_ref();
        res = (res << 1) + node.val;
    }
    res
}
