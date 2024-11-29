use crate::{Iterable, IterableMut};

pub struct Filtered<'a, I, F>
where
    I: Iterable<'a>,
    F: Fn(&I::Item) -> bool,
{
    iterable: I,
    filter: &'a F,
}

impl<'a, I, F> Iterable<'a> for Filtered<'a, I, F>
where
    I: Iterable<'a> + 'a,
    F: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter = FilteredIter<'a, I, F>;

    fn iter(&self) -> Self::Iter {
        FilteredIter {
            iter: self.iterable.iter(),
            filter: &self.filter,
        }
    }
}

pub struct FilteredIter<'a, I, F>
where
    I: Iterable<'a> + 'a,
    F: Fn(&I::Item) -> bool,
{
    iter: I::Iter,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredIter<'a, I, F>
where
    I: Iterable<'a>,
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

pub trait IntoFiltered<'a>
where
    Self: Sized + Iterable<'a>,
{
    fn filtered<F>(self, filter: &'a F) -> Filtered<'_, Self, F>
    where
        F: Fn(&Self::Item) -> bool,
    {
        Filtered {
            iterable: self,
            filter,
        }
    }
}

impl<'a, I> IntoFiltered<'a> for I where I: Iterable<'a> {}

// mut

pub struct FilteredMut<'a, I, F>
where
    I: IterableMut<'a>,
    F: Fn(&I::ItemMut) -> bool,
{
    iterable: I,
    filter: &'a F,
}

pub struct FilteredMutIter<'a, I, F>
where
    I: IterableMut<'a> + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    iter: I::IterMut,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredMutIter<'a, I, F>
where
    I: IterableMut<'a> + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    type Item = I::ItemMut;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            if (self.filter)(&x) {
                return Some(x);
            }
        }
    }
}

impl<'a, I, F> IterableMut<'a> for FilteredMut<'a, I, F>
where
    I: IterableMut<'a> + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    type ItemMut = I::ItemMut;

    type IterMut = FilteredMutIter<'a, I, F>;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        FilteredMutIter {
            iter: self.iterable.iter_mut(),
            filter: &self.filter,
        }
    }
}

pub trait IntoFilteredMut<'a>
where
    Self: Sized + IterableMut<'a>,
{
    fn filtered_mut<F>(self, filter: &'a F) -> FilteredMut<'a, Self, F>
    where
        F: Fn(&Self::ItemMut) -> bool,
    {
        FilteredMut {
            iterable: self,
            filter,
        }
    }
}

impl<'a, I> IntoFilteredMut<'a> for I where I: IterableMut<'a> {}
