use core::cmp::Ordering;

use super::LinkedList;

/////////////////////////////////////////////////////////////////////////
// PartialEq
/////////////////////////////////////////////////////////////////////////
impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

/////////////////////////////////////////////////////////////////////////
// PartialEq
/////////////////////////////////////////////////////////////////////////
impl<T: PartialEq> PartialEq<&[T]> for LinkedList<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len() == other.len() && self.iter().eq(*other)
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.len() != other.len() || self.iter().ne(*other)
    }
}

/////////////////////////////////////////////////////////////////////////
// PartialOrd
/////////////////////////////////////////////////////////////////////////
impl<T: PartialOrd> PartialOrd for LinkedList<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.iter().partial_cmp(other)
    }
}

impl<T: Ord> Ord for LinkedList<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other)
    }
}
