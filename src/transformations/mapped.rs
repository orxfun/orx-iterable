use crate::Iterable;
use std::marker::PhantomData;

pub struct Mapped<I, U, M> {
    iterable: I,
    map: M,
    phantom: PhantomData<U>,
}

impl<I, U, M> Iterable for Mapped<I, U, M>
where
    I: Iterable,
    M: Fn(I::Item) -> U,
{
    type Item = U;

    type Iter<'a> = MappedIter<'a, I, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        MappedIter {
            iter: self.iterable.iter(),
            map: &self.map,
        }
    }
}

pub struct MappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    M: Fn(I::Item) -> U,
{
    iter: I::Iter<'a>,
    map: &'a M,
}

impl<'a, I, U, M> Iterator for MappedIter<'a, I, U, M>
where
    I: Iterable,
    M: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(self.map)
    }
}

pub trait IntoMapped
where
    Self: Iterable + Sized,
{
    fn mapped<U, M>(self, map: M) -> Mapped<Self, U, M>
    where
        M: Fn(Self::Item) -> U,
    {
        Mapped {
            iterable: self,
            map,
            phantom: PhantomData,
        }
    }
}

impl<I> IntoMapped for I where I: Iterable {}
