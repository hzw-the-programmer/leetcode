use crate::lc21::merge_two_lists;
use crate::utils::singly_linked_list::ListNode;

// time : O(L * logk)
// space: O(1)
pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    if lists.len() == 0 {
        return None;
    }

    while lists.len() > 1 {
        let len = lists.len();

        for i in 0..len >> 1 {
            lists[i] = merge_two_lists(lists[i].take(), lists[len - 1 - i].take());
        }

        let mut new_len = len >> 1;
        if len & 1 != 0 {
            new_len += 1;
        }

        unsafe {
            lists.set_len(new_len);
        }
    }

    lists[0].take()
}

// pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
//     let len = lists.len();
//     if len == 0 {
//         return None;
//     }

//     let mut step = 1;
//     while step < len {
//         for i in (0..len - step).step_by(step * 2) {
//             lists[i] = merge_two_lists(lists[i].take(), lists[i + step].take());
//         }
//         step *= 2;
//     }

//     lists[0].take()
// }

// pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
//     if lists.len() == 0 {
//         return None;
//     }

//     while lists.len() > 1 {
//         let len = lists.len();
//         let mut merged = Vec::with_capacity((len + 1) >> 1);

//         for i in 0..len >> 1 {
//             merged.push(merge_two_lists(lists[i].take(), lists[len - 1 - i].take()));
//         }

//         if len & 1 != 0 {
//             merged.push(lists[len >> 1].take());
//         }

//         lists = merged;
//     }

//     lists[0].take()
// }

// pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
//     if lists.len() == 0 {
//         return None;
//     }

//     while lists.len() > 1 {
//         let len = lists.len();
//         let mut merged = Vec::with_capacity((len + 1) >> 1);

//         for i in (1..len).step_by(2) {
//             merged.push(merge_two_lists(lists[i - 1].take(), lists[i].take()));
//         }

//         if len & 1 != 0 {
//             merged.push(lists[len - 1].take());
//         }

//         lists = merged;
//     }

//     lists[0].take()
// }
