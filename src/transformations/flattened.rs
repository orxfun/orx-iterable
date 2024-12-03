use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use std::marker::PhantomData;

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

    fn it(&self) -> Self::Iter {
        self.it.it().flatten()
    }
}

// col

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

    type Iter = core::iter::Flatten<I::Iter<'a>>;

    fn it(&self) -> Self::Iter {
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

    type Iter<'i> = core::iter::Flatten<I::Iter<'i>>
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Flatten<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.it.get_ref().iter().flatten()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().flatten()
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
