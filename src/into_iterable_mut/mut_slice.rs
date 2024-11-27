use super::IntoIterableMut;
use crate::IterableMut;

pub struct IterableMutOfSlice<'a, T>(&'a mut [T]);

impl<'a, T> IterableMut for IterableMutOfSlice<'a, T> {
    type Item = T;

    fn it_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> {
        self.0.iter_mut()
    }
}

impl<'a, T> IntoIterableMut for &'a mut [T] {
    type Item = T;

    fn into_iterable_mut(self) -> impl IterableMut<Item = Self::Item> {
        IterableMutOfSlice(self)
    }
}
