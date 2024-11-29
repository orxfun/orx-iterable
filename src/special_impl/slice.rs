use crate::Iterable;

impl<'a, T> Iterable<'a> for &'a [T] {
    type Item = &'a T;

    type Iter = std::slice::Iter<'a, T>;

    fn iter(&self) -> Self::Iter {
        IntoIterator::into_iter(*self)
    }
}
