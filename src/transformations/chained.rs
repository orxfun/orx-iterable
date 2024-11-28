use crate::{Iterable, IterableMut};

pub struct Chained<I1, I2> {
    it1: I1,
    it2: I2,
}

impl<I1, I2> Iterable for Chained<I1, I2>
where
    I1: Iterable,
    I2: Iterable<Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter<'a> = core::iter::Chain<I1::Iter<'a>, I2::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.iter().chain(self.it2.iter())
    }
}

pub trait IntoChained
where
    Self: Iterable + Sized,
{
    fn chained<I: Iterable>(self, other: I) -> Chained<Self, I> where {
        Chained {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChained for I where I: Iterable {}

// mut

pub struct ChainedMut<'i, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<ItemMut = I1::ItemMut>,
{
    it1: &'i mut I1,
    it2: &'i mut I2,
}

impl<'i, I1, I2> IterableMut for ChainedMut<'i, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<ItemMut = I1::ItemMut>,
{
    type ItemMut = I1::ItemMut;

    type IterMut<'a> = core::iter::Chain<I1::IterMut<'a>, I2::IterMut<'a>> where Self: 'a;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        self.it1.xyz().chain(self.it2.xyz())
    }
}

pub trait IntoChainedMut
where
    Self: IterableMut + Sized,
{
    fn chained_mut<'i, I>(&'i mut self, other: &'i mut I) -> ChainedMut<Self, I>
    where
        I: IterableMut<ItemMut = Self::ItemMut>,
    {
        ChainedMut {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChainedMut for I where I: IterableMut {}
