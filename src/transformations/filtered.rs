use crate::{Iterable, IterableMut, IterableOnce};

pub struct Filtered<I, F> {
    iterable: I,
    filter: F,
}

// impl<I, T, F> IterableOnce for Filtered<I, T, F>
// where
//     F: Fn(&T) -> bool,
//     I: IterableOnce<Item = T>,
// {
//     type Item = T;

//     fn it_once(self) -> impl Iterator<Item = Self::Item> {
//         self.iterable.it_once().filter(self.filter)
//     }
// }

impl<I, F> Iterable for Filtered<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter<'a> = FilteredIter<'a, I, F> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        FilteredIter {
            iter: self.iterable.iter(),
            filter: &self.filter,
        }
    }
}

impl<I, F> IterableOnce for Filtered<I, F>
where
    I: IterableOnce,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter = core::iter::Filter<I::Iter, F>;

    fn it_once(self) -> Self::Iter {
        self.iterable.it_once().filter(self.filter)
    }
}

pub struct FilteredIter<'a, I, F>
where
    I: Iterable + 'a,
    F: Fn(&I::Item) -> bool,
{
    iter: I::Iter<'a>,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredIter<'a, I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => match (self.filter)(&x) {
                true => Some(x),
                false => self.next(),
            },
            None => None,
        }
    }
}

pub trait IntoFiltered
where
    Self: Sized + Iterable,
{
    fn filtered<F>(self, filter: F) -> Filtered<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFiltered for I where I: Iterable {}

// once

pub trait IntoFilteredOnce
where
    Self: Sized + IterableOnce,
{
    fn filtered_once<F>(self, filter: F) -> Filtered<Self, F>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFilteredOnce for I where I: IterableOnce {}

// mut

pub struct FilteredMut<'i, I, F> {
    iterable: &'i mut I,
    filter: F,
}

pub struct FilteredMutIter<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    iter: I::IterMut<'a>,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredMutIter<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    type Item = &'a mut I::ItemMut;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => match (self.filter)(&x) {
                true => Some(x),
                false => self.next(),
            },
            None => None,
        }
    }
}

impl<'i, I, F> IterableMut for FilteredMut<'i, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    type ItemMut = I::ItemMut;

    type IterMut<'a> = FilteredMutIter<'a, I, F> where Self: 'a;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        FilteredMutIter {
            iter: self.iterable.xyz(),
            filter: &self.filter,
        }
    }
}

pub trait IntoFilteredMut
where
    Self: Sized + IterableMut,
{
    fn filtered_mut<F>(&mut self, filter: F) -> FilteredMut<Self, F>
    where
        F: Fn(&Self::ItemMut) -> bool,
    {
        FilteredMut {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFilteredMut for I where I: IterableMut {}
