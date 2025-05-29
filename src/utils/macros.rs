#[macro_export]
macro_rules! nested_vec {
    ([ $($elem:tt),* ]) => {
        vec![ $(nested_vec!($elem)),* ]
    };
    ($($elem:tt),*) => {
        $($elem),*
    }
}
pub use nested_vec;
