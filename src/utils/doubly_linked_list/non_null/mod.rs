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

pub struct IterMut<'a, T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
    marker: PhantomData<&'a Node<T>>,
}

mod basics;

mod iter;
pub use iter::Iter;

mod iter_mut;

mod into_iter;
pub use into_iter::IntoIter;

mod index;

mod extend;

mod partial_eq;
mod partial_ord;

mod other_traits_impl;
