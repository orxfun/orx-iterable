use crate::{Iterable, IterableCol};
use core::marker::PhantomData;
use orx_exclusive::Exclusive;

/// Wraps an `Iterable` and transforms into a fused `Iterable`.
pub struct Fused<I>
where
    I: Iterable,
{
    pub(crate) it: I,
}

impl<I> Iterable for Fused<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter = core::iter::Fuse<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().fuse()
    }
}

// col

/// Wraps an `IterableCol` and transforms into a fused `IterableCol`.
pub struct FusedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    pub(crate) it: E,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a FusedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::Fuse<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().fuse()
    }
}

impl<I, E> IterableCol for FusedCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Fuse<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().fuse()
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
