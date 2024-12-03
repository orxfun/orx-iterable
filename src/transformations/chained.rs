use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use std::marker::PhantomData;

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

    fn iter(&self) -> Self::Iter {
        self.it1.iter().chain(self.it2.iter())
    }
}

// col

pub struct ChainedCol<I1, I2, E1, E2>
where
    I1: IterableCol,
    I2: IterableCol<Item = I1::Item>,
    E1: Exclusive<I1>,
    E2: Exclusive<I2>,
{
    pub(crate) it1: E1,
    pub(crate) it2: E2,
    pub(crate) phantom: PhantomData<(I1, I2)>,
}

impl<I1, I2, E1, E2> IterableCol for ChainedCol<I1, I2, E1, E2>
where
    I1: IterableCol,
    I2: IterableCol<Item = I1::Item>,
    E1: Exclusive<I1>,
    E2: Exclusive<I2>,
{
    type Item = I1::Item;

    type Iter<'i> = core::iter::Chain<I1::Iter<'i>, I2::Iter<'i>>
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Chain<I1::IterMut<'i>, I2::IterMut<'i>>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.get_ref().iter().chain(self.it2.get_ref().iter())
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it1
            .get_mut()
            .iter_mut()
            .chain(self.it2.get_mut().iter_mut())
    }
}
