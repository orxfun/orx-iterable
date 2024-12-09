use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;

/// An iterable which does not yield any element.
pub struct Empty<T> {
    phantom: PhantomData<T>,
}

impl<T> Iterable for Empty<T> {
    type Item = T;

    type Iter = core::iter::Empty<T>;

    fn iter(&self) -> Self::Iter {
        Default::default()
    }
}

impl<T> Iterable for core::iter::Empty<T> {
    type Item = T;

    type Iter = core::iter::Empty<T>;

    fn iter(&self) -> Self::Iter {
        Default::default()
    }
}

// col

/// An iterable collection without any element.
pub struct EmptyCol<T> {
    phantom: PhantomData<T>,
}

impl<'a, T> Iterable for &'a EmptyCol<T> {
    type Item = &'a T;

    type Iter = core::iter::Empty<Self::Item>;

    fn iter(&self) -> Self::Iter {
        Default::default()
    }
}

impl<T> Collection for EmptyCol<T> {
    type Item = T;

    type Iterable<'i>
        = &'i Self
    where
        Self: 'i;

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

impl<T> CollectionMut for EmptyCol<T> {
    type IterMut<'i>
        = core::iter::Empty<&'i mut T>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        Default::default()
    }
}

/// Creates an iterable which does not yield any element.
pub fn empty<T>() -> Empty<T> {
    Empty {
        phantom: PhantomData,
    }
}

/// Creates an iterable collection without any element.
pub fn empty_col<T>() -> EmptyCol<T> {
    EmptyCol {
        phantom: PhantomData,
    }
}
