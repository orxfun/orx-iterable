use crate::{Iterable, IterableMut};

pub struct Skipped<I>
where
    I: Iterable,
{
    skip: usize,
    iterable: I,
}

impl<I> Iterable for Skipped<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter<'a> = core::iter::Skip<I::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().skip(self.skip)
    }
}

pub trait IntoSkipped
where
    Self: Iterable + Sized,
{
    fn skipped(self, num_items_to_skip: usize) -> Skipped<Self> {
        Skipped {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkipped for I where I: Iterable {}

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
