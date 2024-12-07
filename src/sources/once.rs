use crate::{Collection, CollectionMut, Iterable};

/// An iterable which yields a wrapped value only once.
pub struct Once<T>
where
    T: Clone,
{
    value: T,
}

impl<T> Iterable for Once<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::Once<T>;

    fn iter(&self) -> Self::Iter {
        core::iter::once(self.value.clone())
    }
}

impl<T> Iterable for core::iter::Once<T>
where
    T: Clone,
{
    type Item = T;

    type Iter = core::iter::Once<T>;

    fn iter(&self) -> Self::Iter {
        self.clone()
    }
}

// col

/// An iterable collection having only one item.
pub struct OnceCol<T> {
    value: T,
}

impl<'a, T> Iterable for &'a OnceCol<T> {
    type Item = &'a T;

    type Iter = core::iter::Once<Self::Item>;

    fn iter(&self) -> Self::Iter {
        core::iter::once(&self.value)
    }
}

impl<T> Collection for OnceCol<T> {
    type Item = T;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

impl<T> CollectionMut for OnceCol<T> {
    type IterMut<'i> = core::iter::Once<&'i mut T>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        core::iter::once(&mut self.value)
    }
}

/// Creates an iterable which yields only one `value`.
pub fn once<T>(value: T) -> Once<T>
where
    T: Clone,
{
    Once { value }
}

/// Creates an iterable collection having only one element with the given `value`.
pub fn once_col<T>(value: T) -> OnceCol<T> {
    OnceCol { value }
}
