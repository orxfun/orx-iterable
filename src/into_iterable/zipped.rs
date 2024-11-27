use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct Zipped<T, U, I1, I2> {
    it1: I1,
    it2: I2,
    phantom: PhantomData<(T, U)>,
}

impl<T, U, I1, I2> IterableOnce for Zipped<T, U, I1, I2>
where
    I1: IterableOnce<Item = T>,
    I2: IterableOnce<Item = U>,
{
    type Item = (T, U);

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.it1.it_once().zip(self.it2.it_once())
    }
}

impl<T, U, I1, I2> Iterable for Zipped<T, U, I1, I2>
where
    I1: Iterable<Item = T>,
    I2: Iterable<Item = U>,
{
    type Item = (T, U);

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.it1.iter().zip(self.it2.iter())
    }
}

// into

pub trait IntoZippedIterable<T>
where
    Self: Iterable<Item = T>,
{
    fn zipped<I, U>(self, other: I) -> Zipped<T, U, Self, I>
    where
        Self: Sized,
        I: Iterable<Item = U>,
    {
        Zipped {
            it1: self,
            it2: other,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoZippedIterable<T> for I where I: Iterable<Item = T> {}
