use crate::{Iterable, Collection};
use core::marker::PhantomData;
use orx_self_or::SoM;

/// An iterable created by chaining two iterables.
pub struct Chained<I1, I2>
where
    I1: Iterable,
    I2: Iterable<Item = I1::Item>,
{
    pub(crate) it1: I1,
    pub(crate) it2: I2,
}

impl<I1, I2> Iterable for Chained<I1, I2>
where
    I1: Iterable,
    I2: Iterable<Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter = core::iter::Chain<I1::Iter, I2::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.it1.iterate().chain(self.it2.iterate())
    }
}

// col

/// An iterable collection created by chaining two iterable collections.
pub struct ChainedCol<I1, I2, E1, E2>
where
    I1: Collection,
    I2: Collection<Item = I1::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    pub(crate) it1: E1,
    pub(crate) it2: E2,
    pub(crate) phantom: PhantomData<(I1, I2)>,
}

impl<'a, I1, I2, E1, E2> Iterable for &'a ChainedCol<I1, I2, E1, E2>
where
    I1: Collection,
    I2: Collection<Item = I1::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    type Item = &'a I1::Item;

    type Iter = core::iter::Chain<
        <I1::Iterable<'a> as Iterable>::Iter,
        <I2::Iterable<'a> as Iterable>::Iter,
    >;

    fn iterate(&self) -> Self::Iter {
        self.it1.get_ref().iter().chain(self.it2.get_ref().iter())
    }
}

impl<I1, I2, E1, E2> Collection for ChainedCol<I1, I2, E1, E2>
where
    I1: Collection,
    I2: Collection<Item = I1::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    type Item = I1::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Chain<I1::IterMut<'i>, I2::IterMut<'i>>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it1
            .get_mut()
            .iter_mut()
            .chain(self.it2.get_mut().iter_mut())
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
