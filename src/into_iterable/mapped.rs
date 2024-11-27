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

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.iterable.iter().map(&self.map)
    }
}

// into

pub trait IntoMappedIterable<T>
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

impl<T, I> IntoMappedIterable<T> for I where I: Iterable<Item = T> {}

fn abc() {
    //
    let v = vec![1, 2, 3];
}
