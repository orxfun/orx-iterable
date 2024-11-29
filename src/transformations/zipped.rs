use crate::Iterable;
use std::marker::PhantomData;

pub struct Zipped<'a, I1, I2>
where
    I1: Iterable<'a>,
    I2: Iterable<'a>,
{
    it1: I1,
    it2: I2,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I1, I2> Iterable<'a> for Zipped<'a, I1, I2>
where
    I1: Iterable<'a>,
    I2: Iterable<'a>,
{
    type Item = (I1::Item, I2::Item);

    type Iter = std::iter::Zip<I1::Iter, I2::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it1.iter().zip(self.it2.iter())
    }
}

pub trait IntoZipped<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn zipped<I: Iterable<'a>>(self, other: I) -> Zipped<'a, Self, I> {
        Zipped {
            it1: self,
            it2: other,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoZipped<'a> for I where I: Iterable<'a> {}
