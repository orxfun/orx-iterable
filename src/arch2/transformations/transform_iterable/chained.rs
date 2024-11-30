use crate::IterableRef;

pub struct Chained<'a, I1, I2>
where
    I1: IterableRef,
    I2: IterableRef<Item = I1::Item>,
{
    it1: &'a I1,
    it2: &'a I2,
}

impl<'a, I1, I2> IterableRef for Chained<'a, I1, I2>
where
    I1: IterableRef,
    I2: IterableRef<Item = I1::Item>,
{
    type Item = I1::Item;

    type Iter<'i> = core::iter::Chain<I1::Iter<'i>, I2::Iter<'i>> where Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.it1.iter().chain(self.it2.iter())
    }
}

pub trait IntoChained
where
    Self: IterableRef + Sized,
{
    fn chained<'a, I>(&'a self, other: &'a I) -> Chained<'a, Self, I>
    where
        I: IterableRef<Item = Self::Item>,
    {
        Chained {
            it1: self,
            it2: other,
        }
    }
}

impl<I> IntoChained for I where I: IterableRef {}
