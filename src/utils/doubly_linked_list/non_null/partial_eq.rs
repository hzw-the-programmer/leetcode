use super::LinkedList;

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().eq(other)
    }

    fn ne(&self, other: &Self) -> bool {
        self.len() != other.len() || self.iter().ne(other)
    }
}

impl<T: Eq> Eq for LinkedList<T> {}

impl<T: PartialEq> PartialEq<&[T]> for LinkedList<T> {
    fn eq(&self, other: &&[T]) -> bool {
        self.len() == other.len() && self.iter().eq(*other)
    }

    fn ne(&self, other: &&[T]) -> bool {
        self.len() != other.len() || self.iter().ne(*other)
    }
}
