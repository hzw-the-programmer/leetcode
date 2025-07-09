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

mod partial_eq;
mod partial_ord;

mod other_traits_impl;
