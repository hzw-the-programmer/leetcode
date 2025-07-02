use crate::utils::singly_linked_list::ListNode;

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    fn recursive(head: Option<Box<ListNode>>, res: i32) -> i32 {
        match head {
            None => res,
            Some(mut node) => recursive(node.next.take(), (res << 1) + node.val),
        }
    }

    recursive(head, 0)
}
