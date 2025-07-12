#[macro_export]
macro_rules! vec_2d {
    ( $($elem:tt),* ) => {
        vec![ $(vec!$elem),* ]
    };
}
pub use vec_2d;

#[macro_export(local_inner_macros)]
macro_rules! option_array {
    ( $( $tt:tt )* ) => {
        option_array_internal!(@array [] $( $tt )*)
    };
}
pub use option_array;

#[macro_export(local_inner_macros)]
macro_rules! option_array_internal {
    (@array [$($elems:expr),*]) => {
        internal_vec![$($elems),*]
    };

    (@array [$($elems:expr,)*] null $($rest:tt)*) => {
        option_array_internal!(@array [$($elems,)* None] $($rest)*)
    };

    (@array [$($elems:expr,)*] [$($array:expr),*] $($rest:tt)*) => {
        option_array_internal!(@array [$($elems,)* Some(internal_vec![$($array),*])] $($rest)*)
    };

    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        option_array_internal!(@array [$($elems,)* Some($next),] $($rest)*)
    };

    (@array [$($elems:expr,)*] $last:expr) => {
        option_array_internal!(@array [$($elems,)* Some($last)])
    };

    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        option_array_internal!(@array [$($elems,)*] $($rest)*)
    }
}

#[macro_export]
macro_rules! internal_vec {
    ( $( $content:tt )* ) => {
        vec![ $( $content )* ]
    }
}

#[allow(dead_code)]
fn for_cargo_expand() {
    assert_eq!(option_array![1, -2, 3], [Some(1), Some(-2), Some(3)]);
    assert_eq!(
        option_array![1, null, -2, 3],
        [Some(1), None, Some(-2), Some(3)]
    );
    assert_eq!(
        option_array![1, null, -2, 3, null],
        [Some(1), None, Some(-2), Some(3), None]
    );
    assert_eq!(
        option_array![[1], null, [-2], [3]],
        [Some(vec![1]), None, Some(vec![-2]), Some(vec![3])]
    );
}

#[cfg(test)]
mod tests;

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

// #[macro_export]
// macro_rules! option_array {
//     // 入口点：匹配数组字面量
//     ($($input:tt)*) => {
//         $crate::utils::macros::option_array_inner!([] $($input)*)
//     };
// }
// pub use option_array;

// #[macro_export]
// macro_rules! option_array_inner {
//     // 终止条件：所有元素处理完成，返回结果数组
//     ([$($output:expr,)*]) => {
//         [$($output,)*]
//     };

//     // 处理 null 元素（后面有逗号）
//     ([$($output:expr,)*] null, $($rest:tt)*) => {
//         $crate::utils::macros::option_array_inner!([$($output,)* None,] $($rest)*)
//     };

//     // 处理普通表达式元素（后面有逗号）
//     ([$($output:expr,)*] $element:expr, $($rest:tt)*) => {
//         $crate::utils::macros::option_array_inner!([$($output,)* Some($element),] $($rest)*)
//     };

//     // 处理最后一个 null 元素
//     ([$($output:expr,)*] null) => {
//         $crate::utils::macros::option_array_inner!([$($output,)* None,])
//     };

//     // 处理最后一个普通表达式元素
//     ([$($output:expr,)*] $element:expr) => {
//         $crate::utils::macros::option_array_inner!([$($output,)* Some($element),])
//     };
// }
// pub use option_array_inner;
