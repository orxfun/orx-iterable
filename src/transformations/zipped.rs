use crate::Iterable;

/// An iterable created by zipping two iterables.
pub struct Zipped<I1, I2>
where
    I1: Iterable,
    I2: Iterable,
{
    pub(crate) it1: I1,
    pub(crate) it2: I2,
}

impl<I1, I2> Iterable for Zipped<I1, I2>
where
    I1: Iterable,
    I2: Iterable,
{
    type Item = (I1::Item, I2::Item);

    type Iter = core::iter::Zip<I1::Iter, I2::Iter>;

    fn iter(&self) -> Self::Iter {
        self.it1.iter().zip(self.it2.iter())
    }
}
