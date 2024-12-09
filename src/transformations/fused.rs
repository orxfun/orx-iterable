use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

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

/// Wraps an `Collection` and transforms into a fused `Collection`.
pub struct FusedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    pub(crate) it: E,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E> Iterable for &'a FusedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = &'a I::Item;

    type Iter = core::iter::Fuse<<I::Iterable<'a> as Iterable>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it.get_ref().iter().fuse()
    }
}

impl<I, E> Collection for FusedCol<I, E>
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

impl<I, E> CollectionMut for FusedCol<I, E>
where
    I: CollectionMut,
    E: SoM<I>,
{
    type IterMut<'i>
        = core::iter::Fuse<I::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it.get_mut().iter_mut().fuse()
    }
}
