use crate::Iterable;
use std::marker::PhantomData;

pub struct MappedWhile<'a, I, U, M>
where
    I: Iterable<'a>,
    M: Fn(I::Item) -> Option<U>,
{
    iterable: I,
    map_while: &'a M,
    phantom: PhantomData<U>,
}

impl<'a, I, U, M> Iterable<'a> for MappedWhile<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    type Iter = MappedWhileIter<'a, I, U, M>;

    fn iter(&self) -> Self::Iter {
        MappedWhileIter {
            iter: self.iterable.iter(),
            map_while: &self.map_while,
        }
    }
}

pub struct MappedWhileIter<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    iter: I::Iter,
    map_while: &'a M,
}

impl<'a, I, U, M> Iterator for MappedWhileIter<'a, I, U, M>
where
    I: Iterable<'a>,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        (self.map_while)(x)
    }
}

pub trait IntoMappedWhile<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn mapped_while<U, M>(self, map_while: &'a M) -> MappedWhile<'a, Self, U, M>
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

impl<'a, I> IntoMappedWhile<'a> for I where I: Iterable<'a> {}
