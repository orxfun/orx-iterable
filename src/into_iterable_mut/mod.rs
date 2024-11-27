use crate::IterableMut;

pub trait IntoIterableMut {
    type Item;

    fn into_iterable_mut(self) -> impl IterableMut<Item = Self::Item>;
}

mod mut_slice;
