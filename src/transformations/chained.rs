use crate::{Iterable, IterableMut};
use std::marker::PhantomData;

pub struct Chained<'a, I1, I2>
where
    I1: Iterable<'a>,
    I2: Iterable<'a, Item = I1::Item>,
{
    it1: I1,
    it2: I2,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I1, I2> Iterable<'a> for Chained<'a, I1, I2>
where
    I1: Iterable<'a>,
    I2: Iterable<'a, Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter = core::iter::Chain<I1::Iter, I2::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it1.iter().chain(self.it2.iter())
    }
}

pub trait IntoChained<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn chained<I>(self, other: I) -> Chained<'a, Self, I>
    where
        I: Iterable<'a, Item = Self::Item>,
    {
        Chained {
            it1: self,
            it2: other,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoChained<'a> for I where I: Iterable<'a> {}

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

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.it1.iter_mut().chain(self.it2.iter_mut())
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
