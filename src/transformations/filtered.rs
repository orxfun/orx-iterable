use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use std::marker::PhantomData;

pub struct Filtered<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: I,
    pub(crate) filter: P,
}

impl<I, P> Iterable for Filtered<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iter = core::iter::Filter<I::Iter, P>;

    fn it(&self) -> Self::Iter {
        self.it.it().filter(self.filter)
    }
}

// col

pub struct FilteredCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) filter: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a FilteredCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    type Iter = FilteredColIter<'a, I, P>;

    fn it(&self) -> Self::Iter {
        let iter: I::Iter<'_> = self.it.get_ref().iter();
        FilteredColIter {
            iter,
            filter: self.filter,
        }
    }
}

impl<I, E, P> IterableCol for FilteredCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type Iter<'i> = FilteredColIter<'i, I, P>
    where
        Self: 'i;

    type IterMut<'i> = FilteredColIterMut<'i, I, P>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        let iter: I::Iter<'_> = self.it.get_ref().iter();
        FilteredColIter {
            iter,
            filter: self.filter,
        }
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let iter: I::IterMut<'_> = self.it.get_mut().iter_mut();
        FilteredColIterMut {
            iter,
            filter: self.filter,
        }
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

// col - iters

pub struct FilteredColIter<'a, I, P>
where
    I: IterableCol + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: I::Iter<'a>,
    filter: P,
}

impl<'a, I, P> Iterator for FilteredColIter<'a, I, P>
where
    I: IterableCol,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::Iter<'a> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            if (self.filter)(x) {
                return Some(x);
            }
        }
    }
}

pub struct FilteredColIterMut<'a, I, P>
where
    I: IterableCol + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: I::IterMut<'a>,
    filter: P,
}

impl<'a, I, P> Iterator for FilteredColIterMut<'a, I, P>
where
    I: IterableCol,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::IterMut<'a> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            if (self.filter)(x) {
                return Some(x);
            }
        }
    }
}
