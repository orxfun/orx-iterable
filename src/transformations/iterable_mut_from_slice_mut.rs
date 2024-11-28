use crate::IterableMut;

pub struct SliceAsIterableMut<'s, T>(&'s mut [T]);

impl<'s, T> IterableMut for SliceAsIterableMut<'s, T> {
    type ItemMut = T;

    type IterMut<'a> = core::slice::IterMut<'a, T> where Self: 'a;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut()
    }
}

pub trait IntoSliceAsIterableMut<'s, T> {
    fn iterable_mut(self) -> SliceAsIterableMut<'s, T>;
}

impl<'s, T> IntoSliceAsIterableMut<'s, T> for &'s mut [T] {
    fn iterable_mut(self) -> SliceAsIterableMut<'s, T> {
        SliceAsIterableMut(self)
    }
}
