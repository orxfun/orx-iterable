use crate::{Iterable, IterableMut};
use std::marker::PhantomData;

pub struct Skipped<'a, I>
where
    I: Iterable<'a>,
{
    skip: usize,
    iterable: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> Iterable<'a> for Skipped<'a, I>
where
    I: Iterable<'a>,
{
    type Item = <I as Iterable<'a>>::Item;

    type Iter = core::iter::Skip<<I as Iterable<'a>>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.iterable.iter().skip(self.skip)
    }
}

pub trait IntoSkipped<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn skipped(self, num_items_to_skip: usize) -> Skipped<'a, Self> {
        Skipped {
            iterable: self,
            skip: num_items_to_skip,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoSkipped<'a> for I where I: Iterable<'a> {}

// mut

pub struct SkippedMut<'a, I>
where
    I: IterableMut<'a>,
{
    skip: usize,
    iterable: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> IterableMut<'a> for SkippedMut<'a, I>
where
    I: IterableMut<'a>,
{
    type ItemMut = I::ItemMut;

    type IterMut = core::iter::Skip<I::IterMut>;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        self.iterable.iter_mut().skip(self.skip)
    }
}

pub trait IntoSkippedMut<'a>
where
    Self: IterableMut<'a> + Sized,
{
    fn skipped_mut(self, num_items_to_skip: usize) -> SkippedMut<'a, Self> {
        SkippedMut {
            iterable: self,
            skip: num_items_to_skip,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoSkippedMut<'a> for I where I: IterableMut<'a> {}
