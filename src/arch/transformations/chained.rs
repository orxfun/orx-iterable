use crate::{Iterable, IterableMut, IterableRef};
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

// ref

pub struct ChainedRef<'a, I1, I2>
where
    I1: IterableRef,
    I2: IterableRef<ItemRef = I1::ItemRef>,
{
    it1: &'a I1,
    it2: &'a I2,
}

impl<'a, I1, I2> IterableRef for ChainedRef<'a, I1, I2>
where
    I1: IterableRef,
    I2: IterableRef<ItemRef = I1::ItemRef>,
{
    type ItemRef = I1::ItemRef;

    type IterRef<'i> = core::iter::Chain<I1::IterRef<'i>, I2::IterRef<'i>> where Self: 'i;

    fn iter_ref(&self) -> Self::IterRef<'_> {
        self.it1.iter_ref().chain(self.it2.iter_ref())
    }
}

pub trait IntoChainedRef
where
    Self: IterableRef + Sized,
{
    fn chained_ref<'a, I>(&'a self, other: &'a I) -> ChainedRef<'a, Self, I>
    where
        I: IterableRef<ItemRef = Self::ItemRef>,
    {
        ChainedRef {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChainedRef for I where I: IterableRef {}

// mut

pub struct ChainedMut<'a, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<ItemMut = I1::ItemMut>,
{
    it1: &'a mut I1,
    it2: &'a mut I2,
}

impl<'a, I1, I2> IterableMut for ChainedMut<'a, I1, I2>
where
    I1: IterableMut,
    I2: IterableMut<ItemMut = I1::ItemMut>,
{
    type ItemMut = I1::ItemMut;

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
        I: IterableMut<ItemMut = Self::ItemMut>,
    {
        ChainedMut {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChainedMut for I where I: IterableMut {}
