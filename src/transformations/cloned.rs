use crate::Iterable;
use std::marker::PhantomData;

pub struct Cloned<'a, T, I>
where
    I: Iterable<'a, Item = &'a T>,
    T: Clone + 'a,
{
    iterable: I,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I> Iterable<'a> for Cloned<'a, T, I>
where
    I: Iterable<'a, Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type Iter = std::iter::Cloned<I::Iter>;

    fn iter(&self) -> Self::Iter {
        self.iterable.iter().cloned()
    }
}

pub trait IntoCloned<'a, T>
where
    Self: Iterable<'a, Item = &'a T> + Sized,
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
    I: Iterable<'a, Item = &'a T>,
    T: Clone + 'a,
{
}
