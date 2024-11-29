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

pub struct ChainedMut<'a, I1, I2>
where
    I1: IterableMut<'a>,
    I2: IterableMut<'a, ItemMut = I1::ItemMut>,
{
    it1: &'a mut I1,
    it2: &'a mut I2,
}

impl<'a, I1, I2> IterableMut<'a> for ChainedMut<'a, I1, I2>
where
    I1: IterableMut<'a>,
    I2: IterableMut<'a, ItemMut = I1::ItemMut>,
{
    type ItemMut = I1::ItemMut;

    type IterMut = core::iter::Chain<I1::IterMut, I2::IterMut>;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.it1.iter_mut().chain(self.it2.iter_mut())
    }
}

pub trait IntoChainedMut<'a>
where
    Self: IterableMut<'a> + Sized,
{
    fn chained_mut<I>(&'a mut self, other: &'a mut I) -> ChainedMut<'a, Self, I>
    where
        I: IterableMut<'a, ItemMut = Self::ItemMut>,
    {
        ChainedMut {
            it1: self,
            it2: other,
        }
    }
}

impl<'a, I> IntoChainedMut<'a> for I where I: IterableMut<'a> {}
