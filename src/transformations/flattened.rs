use crate::{Iterable, IterableCol};
use core::marker::PhantomData;
use orx_exclusive::Exclusive;

/// Wraps an `Iterable` and creates a new `Iterable` which flattens the elements of
/// the original iterable filtered by a predicate.
pub struct Flattened<I>
where
    I: Iterable,
    I::Item: IntoIterator,
{
    pub(crate) it: I,
}

impl<I> Iterable for Flattened<I>
where
    I: Iterable,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;

    type Iter = core::iter::Flatten<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().flatten()
    }
}

// col

/// Wraps an `IterableCol` and creates a new `IterableCol` which flattens the elements of
/// the original iterable filtered by a predicate.
pub struct FlattenedCol<I, E>
where
    I: IterableCol,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: Exclusive<I>,
{
    pub(crate) it: E,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a FlattenedCol<I, E>
where
    I: IterableCol,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: Exclusive<I>,
{
    type Item = &'a <I::Item as IntoIterator>::Item;

    type Iter = core::iter::Flatten<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().flatten()
    }
}

impl<I, E> IterableCol for FlattenedCol<I, E>
where
    I: IterableCol,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: Exclusive<I>,
{
    type Item = <I::Item as IntoIterator>::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Flatten<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().flatten()
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
