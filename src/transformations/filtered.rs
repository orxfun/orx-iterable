use crate::{Iterable, IterableMut};

pub struct Filtered<I, F>
where
    I: Iterable,
    F: Fn(&I::Item) -> bool,
{
    iterable: I,
    filter: F,
}

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
        loop {
            let x = self.iter.next()?;
            if (self.filter)(&x) {
                return Some(x);
            }
        }
    }
}

pub trait IntoFiltered
where
    Self: Sized + Iterable,
{
    fn filtered<F>(self, filter: F) -> Filtered<Self, F>
    where
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFiltered for I where I: Iterable {}

// mut

pub struct FilteredMut<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    iterable: &'a mut I,
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
        loop {
            let x = self.iter.next()?;
            if (self.filter)(&x) {
                return Some(x);
            }
        }
    }
}

impl<'a, I, F> IterableMut for FilteredMut<'a, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    type ItemMut = I::ItemMut;

    type IterMut<'b> = FilteredMutIter<'b, I, F> where Self: 'b;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        FilteredMutIter {
            iter: self.iterable.iter_mut(),
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
