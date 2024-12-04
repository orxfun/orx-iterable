use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use core::marker::PhantomData;

pub struct Reversed<I>
where
    I: Iterable,
    I::Iter: DoubleEndedIterator,
{
    pub(crate) it: I,
}

impl<I> Iterable for Reversed<I>
where
    I: Iterable,
    I::Iter: DoubleEndedIterator,
{
    type Item = I::Item;

    type Iter = core::iter::Rev<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().rev()
    }
}

// col

pub struct ReversedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
    for<'b> I::IterMut<'b>: DoubleEndedIterator,
{
    pub(crate) it: E,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a ReversedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
    for<'b> I::IterMut<'b>: DoubleEndedIterator,
{
    type Item = &'a I::Item;

    type Iter = core::iter::Rev<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().rev()
    }
}

impl<I, E> IterableCol for ReversedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
    for<'b> I::IterMut<'b>: DoubleEndedIterator,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Rev<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().rev()
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
