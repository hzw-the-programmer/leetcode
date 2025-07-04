use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::rc::Rc;

pub type Tree = Option<Rc<RefCell<TreeNode>>>;

#[derive(PartialEq, Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Tree,
    pub right: Tree,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn new(val: i32, left: Tree, right: Tree) -> Tree {
    Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
}
pub use new as new_tree;

// pub fn from(arr: &[Option<i32>]) -> Tree {
//     from_recursive(arr, 0)
// }

// fn from_recursive(arr: &[Option<i32>], index: usize) -> Tree {
//     if index < arr.len() && arr[index].is_some() {
//         new(
//             arr[index].unwrap(),
//             from_recursive(arr, 2 * index + 1),
//             from_recursive(arr, 2 * index + 2),
//         )
//     } else {
//         None
//     }
// }

pub fn from_slice(mut values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    values = &values[1..];

    let mut queue = VecDeque::from(vec![root.clone()]);
    while let Some(node) = queue.pop_front() {
        let node = node.borrow_mut();
        let children = RefMut::map_split(node, |node| (&mut node.left, &mut node.right));
        let children = vec![children.0, children.1];

        for mut child in children {
            if values.is_empty() {
                break;
            }
            if let Some(val) = values[0] {
                let node = Rc::new(RefCell::new(TreeNode::new(val)));
                *child = Some(node.clone());
                queue.push_back(node);
            }
            values = &values[1..];
        }
    }

    Some(root)
}

#[macro_export]
macro_rules! option_array {
    // 入口点：匹配数组字面量
    ([$($input:tt)*]) => {
        $crate::utils::binary_tree::option_array_inner!([] $($input)*)
    };
}
pub use option_array;

#[macro_export]
macro_rules! option_array_inner {
    // 终止条件：所有元素处理完成，返回结果数组
    ([$($output:expr,)*]) => {
        [$($output,)*]
    };

    // 处理 null 元素（后面有逗号）
    ([$($output:expr,)*] null, $($rest:tt)*) => {
        $crate::utils::binary_tree::option_array_inner!([$($output,)* None,] $($rest)*)
    };

    // 处理普通表达式元素（后面有逗号）
    ([$($output:expr,)*] $element:expr, $($rest:tt)*) => {
        $crate::utils::binary_tree::option_array_inner!([$($output,)* Some($element),] $($rest)*)
    };

    // 处理最后一个 null 元素
    ([$($output:expr,)*] null) => {
        option_array_inner!([$($output,)* None,])
    };

    // 处理最后一个普通表达式元素
    ([$($output:expr,)*] $element:expr) => {
        $crate::utils::binary_tree::option_array_inner!([$($output,)* Some($element),])
    };
}
pub use option_array_inner;

// #[macro_export]
// macro_rules! build {
//     ($($input:tt)*) => {
//         $crate::utils::binary_tree::from(&$crate::utils::binary_tree::option_array!($($input)*))
//     };
// }
// pub use build;

// #[macro_export]
// macro_rules! btree {
//     ($($input:tt)*) => {
//         $crate::utils::binary_tree::from(&$crate::utils::binary_tree::option_array!([$($input)*]))
//     };
// }
// pub use btree;

#[macro_export]
macro_rules! btree {
    ($($input:tt)*) => {
        $crate::utils::binary_tree::from_slice(&$crate::utils::binary_tree::option_array!([$($input)*]))
    };
}
pub use btree;

// macro_rules! binary_tree {
//     ([ $($elem:tt),* ]) => {{
//         let elements = [ $(binary_tree!(@parse $elem)),* ];
//         binary_tree!(@build 0, &elements)
//     }};
//     (@parse null) => {
//         None
//     };
//     (@parse $num:expr) => {
//         Some($num)
//     };
//     (@build $index:expr, $elements:expr) => {{
//         if $index >= $elements.len() {
//             None
//         } else {
//             if let Some(val) = $elements[$index] {
//                 // $crate::utils::binary_tree::new_tree(val, None, None)
//                 $crate::utils::binary_tree::new_tree(val, binary_tree!(@build 2 * $index + 1, $elements), binary_tree!(@build 2 * $index + 2, $elements))
//             } else {
//                 None
//             }
//         }
//     }}
// }

// macro_rules! binary_tree {
//     // 入口点：匹配数组并初始化递归处理
//     ([ $($elements:tt),* ]) => {{
//         let elements = [$($elements),*];
//         binary_tree!(@build 0, &elements)
//     }};

//     // 递归构建节点：通过索引访问数组元素
//     (@build $index:expr, $elements:expr) => {{
//         if $index >= $elements.len() || $elements[$index] == "null" {
//             None
//         } else {
//             let val = $elements[$index].parse::<i32>().unwrap();
//             Some(Box::new(TreeNode {
//                 val,
//                 left: binary_tree!(@build 2 * $index + 1, $elements),
//                 right: binary_tree!(@build 2 * $index + 2, $elements),
//             }))
//         }
//     }};
// }

// macro_rules! option_array {
//     // 主规则：匹配数组并处理每个元素
//     ([ $($elem:tt),* ]) => {
//         [ $(option_array!(@parse $elem)),* ]
//     };
//     // 内部规则：处理 `null` 转换为 `None`
//     (@parse null) => {
//         None
//     };
//     // 内部规则：处理非 `null` 元素，包裹为 `Some`
//     (@parse $num:expr) => {
//         Some($num)
//     };
// }

#[cfg(test)]
mod tests;
