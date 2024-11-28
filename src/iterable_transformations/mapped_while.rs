use crate::Iterable;
use std::marker::PhantomData;

// iterable

pub struct MappedWhile<I, T, U, M>
where
    M: Fn(T) -> Option<U>,
{
    iterable: I,
    map_while: M,
    phantom: PhantomData<T>,
}

impl<I, T, U, M> Iterable for MappedWhile<I, T, U, M>
where
    M: Fn(T) -> Option<U>,
    I: Iterable<Item = T>,
{
    type Item = U;

    type Iter<'a> = MappedWhileIter<'a, I, T, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        MappedWhileIter {
            iter: self.iterable.iter(),
            map_while: &self.map_while,
        }
    }
}

// iter

pub struct MappedWhileIter<'a, I, T, U, M>
where
    M: Fn(T) -> Option<U>,
    I: Iterable<Item = T> + 'a,
{
    iter: I::Iter<'a>,
    map_while: &'a M,
}

impl<'a, I, T, U, M> Iterator for MappedWhileIter<'a, I, T, U, M>
where
    M: Fn(T) -> Option<U>,
    I: Iterable<Item = T>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        (self.map_while)(x)
    }
}

// into

pub trait IntoMappedWhileIterable<T>
where
    Self: Iterable<Item = T>,
{
    fn mapped_while<U, M>(self, map_while: M) -> MappedWhile<Self, T, U, M>
    where
        M: Fn(T) -> Option<U>,
        Self: Sized,
    {
        MappedWhile {
            iterable: self,
            map_while,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoMappedWhileIterable<T> for I where I: Iterable<Item = T> {}
