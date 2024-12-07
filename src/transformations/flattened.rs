use crate::{Iterable, Collection};
use core::marker::PhantomData;
use orx_self_or::SoM;

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

    fn iterate(&self) -> Self::Iter {
        self.it.iterate().flatten()
    }
}

// col

/// Wraps an `Collection` and creates a new `Collection` which flattens the elements of
/// the original iterable filtered by a predicate.
pub struct FlattenedCol<I, E>
where
    I: Collection,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
{
    pub(crate) it: E,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a FlattenedCol<I, E>
where
    I: Collection,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
{
    type Item = &'a <I::Item as IntoIterator>::Item;

    type Iter = core::iter::Flatten<<I::Iterable<'a> as Iterable>::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.it.get_ref().iter().flatten()
    }
}

impl<I, E> Collection for FlattenedCol<I, E>
where
    I: Collection,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
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
