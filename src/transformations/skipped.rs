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
    I: IterableMut,
{
    skip: usize,
    iterable: &'a mut I,
}

impl<'a, I> IterableMut for SkippedMut<'a, I>
where
    I: IterableMut,
{
    type ItemMut = I::ItemMut;

    type IterMut<'b> = core::iter::Skip<I::IterMut<'b>> where Self: 'b;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iterable.iter_mut().skip(self.skip)
    }
}

pub trait IntoSkippedMut
where
    Self: IterableMut + Sized,
{
    fn skipped_mut(&mut self, num_items_to_skip: usize) -> SkippedMut<Self> {
        SkippedMut {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkippedMut for I where I: IterableMut {}
