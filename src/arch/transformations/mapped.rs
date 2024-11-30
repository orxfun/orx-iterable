use crate::Iterable;
use std::marker::PhantomData;

pub struct Mapped<'a, I, U, M>
where
    I: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    iterable: I,
    map: &'a M,
    phantom: PhantomData<U>,
}

impl<'a, I, U, M> Iterable<'a> for Mapped<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> U,
{
    type Item = U;

    type Iter = MappedIter<'a, I, U, M>;

    fn iter(&self) -> Self::Iter {
        MappedIter {
            iter: self.iterable.iter(),
            map: &self.map,
        }
    }
}

pub struct MappedIter<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> U,
{
    iter: I::Iter,
    map: &'a M,
}

impl<'a, I, U, M> Iterator for MappedIter<'a, I, U, M>
where
    I: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(self.map)
    }
}

pub trait IntoMapped<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn mapped<U, M>(self, map: &'a M) -> Mapped<'a, Self, U, M>
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

impl<'a, I> IntoMapped<'a> for I where I: Iterable<'a> {}
