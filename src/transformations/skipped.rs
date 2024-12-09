use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

/// Wraps an `Iterable` and creates a new `Iterable` which skips first `n` the elements
/// of the original iterable.
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

/// Wraps an `Collection` and creates a new `Collection` which skips first `n` the elements
/// of the original iterable.
pub struct SkippedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    pub(crate) it: E,
    pub(crate) n: usize,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a SkippedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::Skip<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().skip(self.n)
    }
}

impl<I, E> Collection for SkippedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = I::Item;

    type Iterable<'i>
        = &'i Self
    where
        Self: 'i;

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

impl<I, E> CollectionMut for SkippedCol<I, E>
where
    I: CollectionMut,
    E: SoM<I>,
{
    type IterMut<'i>
        = core::iter::Skip<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().skip(self.n)
    }
}
