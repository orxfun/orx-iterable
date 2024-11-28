use crate::{Iterable, IterableMut, IterableOnce};

pub struct Skipped<I> {
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
    Self: Iterable,
{
    fn skipped(self, num_items_to_skip: usize) -> Skipped<Self>
    where
        Self: Sized,
    {
        Skipped {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkipped for I where I: Iterable {}

// once

impl<I> IterableOnce for Skipped<I>
where
    I: IterableOnce,
{
    type Item = I::Item;

    type Iter = core::iter::Skip<I::Iter>;

    fn it_once(self) -> Self::Iter {
        self.iterable.it_once().skip(self.skip)
    }
}

pub trait IntoSkippedOnce
where
    Self: IterableOnce,
{
    fn skipped_once(self, num_items_to_skip: usize) -> Skipped<Self>
    where
        Self: Sized,
    {
        Skipped {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkippedOnce for I where I: IterableOnce {}

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

    fn xyz(&mut self) -> Self::IterMut<'_> {
        self.iterable.xyz().skip(self.skip)
    }
}

pub trait IntoSkippedMut
where
    Self: IterableMut,
{
    fn skipped_mut(&mut self, num_items_to_skip: usize) -> SkippedMut<Self>
    where
        Self: Sized,
    {
        SkippedMut {
            iterable: self,
            skip: num_items_to_skip,
        }
    }
}

impl<I> IntoSkippedMut for I where I: IterableMut {}
