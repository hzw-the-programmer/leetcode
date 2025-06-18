use super::path_sum;
use crate::utils::binary_tree::btree;
use crate::utils::macros::vec_2d;

#[test]
fn test() {
    assert_eq!(
        path_sum(btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, 5, 1], 22),
        vec_2d![[5, 4, 11, 2], [5, 8, 4, 5]]
    );
    // assert_eq!(
    //     path_sum(
    //         btree![5, 4, 8, 11, null, 13, 4, 7, 2, null, null, null, null, 5, 1],
    //         22
    //     ),
    //     vec_2d![[5, 8, 4, 5], [5, 4, 11, 2]]
    // );
    assert_eq!(path_sum(btree![1, 2, 3], 5), Vec::<Vec<i32>>::new());
    assert_eq!(path_sum(btree![1, 2], 0), vec![vec![0i32; 0]; 0]);
}
