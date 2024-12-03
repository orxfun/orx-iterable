use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use std::marker::PhantomData;

pub struct Skipped<I>
where
    I: Iterable,
{
    pub(crate) it: I,
    pub(crate) n: usize,
}

impl<I> Iterable for Skipped<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter = core::iter::Skip<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().skip(self.n)
    }
}

// col

pub struct SkippedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    pub(crate) it: E,
    pub(crate) n: usize,
    pub(crate) phantom: PhantomData<I>,
}

impl<I, E> IterableCol for SkippedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = I::Item;

    type Iter<'i> = core::iter::Skip<I::Iter<'i>>
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Skip<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.it.get_ref().iter().skip(self.n)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().skip(self.n)
    }
}
