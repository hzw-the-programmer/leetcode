#[macro_export]
macro_rules! vec_2d {
    ( $($elem:tt),* ) => {
        vec![ $(vec!$elem),* ]
    };
}

// #[macro_export]
// macro_rules! nested_vec {
//     // 匹配外层数组语法，处理内层数组
//     ([ $($inner:tt),* ]) => {
//         vec![ $(
//             nested_vec!(@inner $inner)
//         ),* ]
//     };
//     // 处理内层数组（递归调用）
//     (@inner [$($x:expr),* $(,)?]) => {
//         vec![$($x),*]
//     };
// }

pub use vec_2d;
