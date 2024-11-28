use crate::Iterable;
use std::marker::PhantomData;

pub struct MappedWhile<I, U, M> {
    iterable: I,
    map_while: M,
    phantom: PhantomData<U>,
}

impl<I, U, M> Iterable for MappedWhile<I, U, M>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    type Iter<'a> = MappedWhileIter<'a, I, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        MappedWhileIter {
            iter: self.iterable.iter(),
            map_while: &self.map_while,
        }
    }
}

pub struct MappedWhileIter<'a, I, U, M>
where
    I: Iterable + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    iter: I::Iter<'a>,
    map_while: &'a M,
}

impl<'a, I, U, M> Iterator for MappedWhileIter<'a, I, U, M>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        (self.map_while)(x)
    }
}

pub trait IntoMappedWhile
where
    Self: Iterable + Sized,
{
    fn mapped_while<U, M>(self, map_while: M) -> MappedWhile<Self, U, M>
    where
        M: Fn(Self::Item) -> Option<U>,
    {
        MappedWhile {
            iterable: self,
            map_while,
            phantom: PhantomData,
        }
    }
}

impl<I> IntoMappedWhile for I where I: Iterable {}
