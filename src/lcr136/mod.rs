// LCR 136. 删除链表的节点

// mod recursive;
// pub use recursive::delete_node;

mod iterative;
pub use iterative::delete_node;

#[cfg(test)]
mod tests {
    use super::delete_node;
    use crate::utils::singly_linked_list::list;

    #[test]
    fn t1() {
        let tests = [
            (list![4, 5, 1, 9], 5, list![4, 1, 9]),
            (list![4, 5, 1, 9], 1, list![4, 5, 9]),
        ];

        for (i, test) in tests.iter().enumerate() {
            assert_eq!(delete_node(test.0.clone(), test.1), test.2, "{}", i);
        }
    }
}
