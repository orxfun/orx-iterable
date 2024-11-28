use crate::Iterable;

pub struct Zipped<I1, I2>
where
    I1: Iterable,
    I2: Iterable,
{
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
