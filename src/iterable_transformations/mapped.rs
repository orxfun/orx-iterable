use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

// iterable

pub struct Mapped<I, T, U, M>
where
    M: Fn(T) -> U,
{
    iterable: I,
    map: M,
    phantom: PhantomData<T>,
}

impl<I, T, U, M> IterableOnce for Mapped<I, T, U, M>
where
    M: Fn(T) -> U,
    I: IterableOnce<Item = T>,
{
    type Item = U;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.iterable.it_once().map(self.map)
    }
}

impl<I, T, U, M> Iterable for Mapped<I, T, U, M>
where
    M: Fn(T) -> U,
    I: Iterable<Item = T>,
{
    type Item = U;

    type Iter<'a> = MappedIter<'a, I, T, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        MappedIter {
            iter: self.iterable.iter(),
            map: &self.map,
        }
    }
}

// iter

pub struct MappedIter<'a, I, T, U, M>
where
    M: Fn(T) -> U,
    I: Iterable<Item = T> + 'a,
{
    iter: I::Iter<'a>,
    map: &'a M,
}

impl<'a, I, T, U, M> Iterator for MappedIter<'a, I, T, U, M>
where
    M: Fn(T) -> U,
    I: Iterable<Item = T>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(self.map)
    }
}

// into

pub trait IntoMapped<T>
where
    Self: Iterable<Item = T>,
{
    fn mapped<U, M>(self, map: M) -> Mapped<Self, T, U, M>
    where
        M: Fn(T) -> U,
        Self: Sized,
    {
        Mapped {
            iterable: self,
            map,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoMapped<T> for I where I: Iterable<Item = T> {}
