use crate::{Iterable, IterableCol};
use core::marker::PhantomData;
use orx_exclusive::Exclusive;

/// Wraps an `Iterable` and creates a new `Iterable` which yields only the first `n` the elements
/// of the original iterable.
pub struct Taken<I>
where
    I: Iterable,
{
    pub(crate) it: I,
    pub(crate) n: usize,
}

impl<I> Iterable for Taken<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter = core::iter::Take<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().take(self.n)
    }
}

// col

/// Wraps an `IterableCol` and creates a new `IterableCol` which yields only the first `n` the elements
/// of the original iterable.
pub struct TakenCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    pub(crate) it: E,
    pub(crate) n: usize,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a TakenCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::Take<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().take(self.n)
    }
}

impl<I, E> IterableCol for TakenCol<I, E>
where
    I: IterableCol,
    E: Exclusive<I>,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Take<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().take(self.n)
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
