use crate::{Iterable, IterableOnce};

pub struct Zipped<I1, I2> {
    it1: I1,
    it2: I2,
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
    Self: Iterable + Sized,
{
    fn zipped<I: Iterable>(self, other: I) -> Zipped<Self, I> {
        Zipped {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoZipped for I where I: Iterable {}

// once

impl<I1, I2> IterableOnce for Zipped<I1, I2>
where
    I1: IterableOnce,
    I2: IterableOnce,
{
    type Item = (I1::Item, I2::Item);

    type Iter = std::iter::Zip<I1::Iter, I2::Iter>;

    fn it_once(self) -> Self::Iter {
        self.it1.it_once().zip(self.it2.it_once())
    }
}

pub trait IntoZippedOnce
where
    Self: IterableOnce + Sized,
{
    fn zipped_once<I: IterableOnce>(self, other: I) -> Zipped<Self, I> {
        Zipped {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoZippedOnce for I where I: IterableOnce {}
