use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct Cloned<'a, T, I>
where
    T: Clone + 'a,
{
    iterable: I,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I> Cloned<'a, T, I>
where
    T: Clone + 'a,
{
    pub fn into_inner(self) -> I {
        self.iterable
    }
}

impl<'a, T, I> IterableOnce for Cloned<'a, T, I>
where
    I: IterableOnce<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    type Iter = std::iter::Cloned<I::Iter>;

    fn it_once(self) -> Self::Iter {
        self.iterable.it_once().cloned()
    }
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

// once

pub trait IntoClonedOnce<'a, T>
where
    Self: IterableOnce<Item = &'a T> + Sized,
    T: Clone + 'a,
{
    fn cloned_once(self) -> Cloned<'a, T, Self> {
        Cloned {
            iterable: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I> IntoClonedOnce<'a, T> for I
where
    I: IterableOnce<Item = &'a T>,
    T: Clone + 'a,
{
}
