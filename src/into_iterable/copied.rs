use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct Copied<'a, T, I>
where
    T: Copy + 'a,
{
    iterable: I,
    phantom: PhantomData<&'a T>,
}

impl<'a, T, I> Copied<'a, T, I>
where
    T: Copy + 'a,
{
    pub fn into_inner(self) -> I {
        self.iterable
    }
}

impl<'a, T, I> IterableOnce for Copied<'a, T, I>
where
    I: IterableOnce<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.iterable.it_once().copied()
    }
}

impl<'a, T, I> Iterable for Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.iterable.iter().copied()
    }
}

// into

pub trait IntoCopiedIterable<'a, T>
where
    Self: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    fn copied(self) -> Copied<'a, T, Self>
    where
        Self: Sized,
    {
        Copied {
            iterable: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, I> IntoCopiedIterable<'a, T> for I
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
}
