use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct Zipped<I1, I2> {
    it1: I1,
    it2: I2,
}

impl<I1, I2> IterableOnce for Zipped<I1, I2>
where
    I1: IterableOnce,
    I2: IterableOnce,
{
    type Item = (I1::Item, I2::Item);

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.it1.it_once().zip(self.it2.it_once())
    }
}

impl<I1, I2> Iterable for Zipped<I1, I2>
where
    I1: Iterable,
    I2: Iterable,
{
    type Item = (I1::Item, I2::Item);

    type Iter<'a> = std::iter::Zip<I1::Iter<'a>, I2::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.iter().zip(self.it2.iter())
    }
}

pub trait IntoZipped
where
    Self: Iterable,
{
    fn zipped<I>(self, other: I) -> Zipped<Self, I>
    where
        Self: Sized,
        I: Iterable,
    {
        Zipped {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoZipped for I where I: Iterable {}

// mut

pub struct ZippedMut<'a, T, U, I1, I2> {
    it1: &'a mut I1,
    it2: &'a mut I2,
    phantom: PhantomData<(T, U)>,
}
