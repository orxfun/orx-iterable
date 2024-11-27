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

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.iterable.it_once().cloned()
    }
}

impl<'a, T, I> Iterable for Cloned<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.iterable.iter().cloned()
    }
}

// into

pub trait IntoClonedIterable<'a, T>
where
    Self: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
    fn cloned(self) -> Cloned<'a, T, Self>
    where
        Self: Sized,
    {
        Cloned {
            iterable: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I> IntoClonedIterable<'a, T> for I
where
    I: Iterable<Item = &'a T>,
    T: Clone + 'a,
{
}
