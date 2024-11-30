use crate::{IterableRef, IterableMut};

pub struct ChainedMut<'a, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<Item = I1::Item>,
{
    it1: &'a mut I1,
    it2: &'a mut I2,
}

impl<'a, I1, I2> IterableRef for ChainedMut<'a, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter<'i> = core::iter::Chain<I1::Iter<'i>, I2::Iter<'i>> where Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.iter().chain(self.it2.iter())
    }
}

impl<'a, I1, I2> IterableMut for ChainedMut<'a, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<Item = I1::Item>,
{
    type IterMut<'i> = core::iter::Chain<I1::IterMut<'i>, I2::IterMut<'i>> where Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it1.iter_mut().chain(self.it2.iter_mut())
    }
}

pub trait IntoChainedMut
where
    Self: IterableMut + Sized,
{
    fn chained_mut<'a, I>(&'a mut self, other: &'a mut I) -> ChainedMut<'a, Self, I>
    where
        I: IterableMut<Item = Self::Item>,
    {
        ChainedMut {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChainedMut for I where I: IterableMut {}
