use crate::Iterable;
use std::marker::PhantomData;

pub struct Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    iterable: I,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I> Iterable for Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type Iter<'b> = std::iter::Cloned<I::Iter<'b>> where Self: 'b;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().cloned()
    }
}

pub trait IntoCloned<'a, T>
where
    Self: Iterable<Item = &'a T> + Sized,
    T: Clone + 'a,
{
    fn cloned(self) -> Cloned<'a, T, Self> {
        Cloned {
            iterable: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I> IntoCloned<'a, T> for I
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
}
