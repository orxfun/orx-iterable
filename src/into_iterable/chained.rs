use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct Chained<T, I1, I2> {
    it1: I1,
    it2: I2,
    phantom: PhantomData<T>,
}

impl<T, I1, I2> IterableOnce for Chained<T, I1, I2>
where
    I1: IterableOnce<Item = T>,
    I2: IterableOnce<Item = T>,
{
    type Item = T;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.it1.it_once().chain(self.it2.it_once())
    }
}

impl<T, I1, I2> Iterable for Chained<T, I1, I2>
where
    I1: Iterable<Item = T>,
    I2: Iterable<Item = T>,
{
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.it1.iter().chain(self.it2.iter())
    }
}

// into

pub trait IntoChainedIterable<T>
where
    Self: Iterable<Item = T>,
{
    fn chained<I>(self, other: I) -> Chained<T, Self, I>
    where
        Self: Sized,
        I: Iterable<Item = T>,
    {
        Chained {
            it1: self,
            it2: other,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoChainedIterable<T> for I where I: Iterable<Item = T> {}
