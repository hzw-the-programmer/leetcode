use core::fmt;
use core::marker::PhantomData;
use core::ptr::NonNull;

pub struct Node<T> {
    pub val: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

mod basics;

mod iter;
pub struct Iter<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

mod iter_mut;
pub use iter_mut::IterMut;

mod into_iter;
pub use into_iter::IntoIter;

mod index;

mod extend;

mod eq_ord;

/////////////////////////////////////////////////////////////////////////
// Debug
/////////////////////////////////////////////////////////////////////////
impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
