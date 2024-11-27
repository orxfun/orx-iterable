use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

// iterable

pub struct Filtered<I, T, F>
where
    F: Fn(&T) -> bool,
{
    iterable: I,
    filter: F,
    phantom: PhantomData<T>,
}

impl<I, T, F> IterableOnce for Filtered<I, T, F>
where
    F: Fn(&T) -> bool,
    I: IterableOnce<Item = T>,
{
    type Item = T;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.iterable.it_once().filter(self.filter)
    }
}

impl<I, T, F> Iterable for Filtered<I, T, F>
where
    F: Fn(&T) -> bool,
    I: Iterable<Item = T>,
{
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.iterable.iter().filter(&self.filter)
    }
}

// into

pub trait IntoFilteredIterable<T>
where
    Self: Iterable<Item = T>,
{
    fn filtered<F>(self, filter: F) -> Filtered<Self, T, F>
    where
        F: Fn(&T) -> bool,
        Self: Sized,
    {
        Filtered {
            iterable: self,
            filter,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoFilteredIterable<T> for I where I: Iterable<Item = T> {}
