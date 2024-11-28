use crate::IterableMut;

pub struct IterableSliceMut<'a, T>(&'a mut [T]);

impl<'a, T> IterableMut for IterableSliceMut<'a, T> {
    type Item = T;

    type IterMut<'b> = std::slice::IterMut<'b, T> where Self: 'b;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.0.iter_mut()
    }
}

impl<'a, T> IntoIterableSliceMut for &'a mut [T] {
    type Item = T;

    fn iterable_mut(self) -> impl IterableMut<Item = Self::Item> {
        IterableSliceMut(self)
    }
}

// into

pub trait IntoIterableSliceMut {
    type Item;

    fn iterable_mut(self) -> impl IterableMut<Item = Self::Item>;
}
