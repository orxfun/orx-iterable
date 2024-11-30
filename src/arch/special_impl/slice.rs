use crate::{Iterable, IterableMut};

impl<'a, T> Iterable<'a> for &'a [T] {
    type Item = &'a T;

    type Iter = core::slice::Iter<'a, T>;

    fn iter(&self) -> Self::Iter {
        IntoIterator::into_iter(*self)
    }
}

impl<'a, T> IterableMut<'a> for &'a mut [T] {
    type ItemMut = &'a mut T;

    type IterMut = core::slice::IterMut<'a, T>;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.into_iter()
    }
}
