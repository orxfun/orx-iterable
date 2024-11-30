use crate::{Iterable, IterableMut, IterableRef};

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

// ref

pub struct FilteredRef<'a, I, F>
where
    I: IterableRef,
    F: Fn(&I::ItemRef) -> bool,
{
    iterable: &'a I,
    filter: &'a F,
}

pub struct FilteredRefIter<'a, I, F>
where
    I: IterableRef + 'a,
    F: Fn(&I::ItemRef) -> bool,
{
    iter: I::IterRef<'a>,
    filter: &'a F,
}

impl<'a, I, F> Iterator for FilteredRefIter<'a, I, F>
where
    I: IterableRef + 'a,
    F: Fn(&I::ItemRef) -> bool,
{
    type Item = &'a I::ItemRef;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            if (self.filter)(&x) {
                return Some(x);
            }
        }
    }
}

impl<'a, I, F> IterableRef for FilteredRef<'a, I, F>
where
    I: IterableRef + 'a,
    F: Fn(&I::ItemRef) -> bool,
{
    type ItemRef = I::ItemRef;

    type IterRef<'b> = FilteredRefIter<'b, I, F> where Self: 'b;

    fn iter_ref(&self) -> Self::IterRef<'_> {
        FilteredRefIter {
            iter: self.iterable.iter_ref(),
            filter: &self.filter,
        }
    }
}

pub trait IntoFilteredRef
where
    Self: Sized + IterableRef,
{
    fn filtered_ref<'a, F>(&'a self, filter: &'a F) -> FilteredRef<'a, Self, F>
    where
        F: Fn(&Self::ItemRef) -> bool,
    {
        FilteredRef {
            iterable: self,
            filter,
        }
    }
}

impl<I> IntoFilteredRef for I where I: IterableRef {}

// mut

pub struct FilteredMut<'a, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    iterable: &'a mut I,
    filter: &'a F,
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
    I: IterableMut + 'a,
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
    fn filtered_mut<'a, F>(&'a mut self, filter: &'a F) -> FilteredMut<'a, Self, F>
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
