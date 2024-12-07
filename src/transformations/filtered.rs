use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

/// Wraps an `Iterable` and creates a new `Iterable` which yields elements of
/// the original iterable filtered by a predicate.
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

    fn iter(&self) -> Self::Iter {
        self.it.iter().filter(self.filter)
    }
}

// col

/// Wraps an `Collection` and creates a new `Collection` which yields elements of
/// the original iterable filtered by a predicate.
pub struct FilteredCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) filter: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a FilteredCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    type Iter = FilteredColIter<'a, I, P>;

    fn iter(&self) -> Self::Iter {
        let iter = self.it.get_ref().iter();
        FilteredColIter {
            iter,
            filter: self.filter,
        }
    }
}

impl<I, E, P> Collection for FilteredCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

impl<I, E, P> CollectionMut for FilteredCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type IterMut<'i> = FilteredColIterMut<'i, I, P>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let iter: I::IterMut<'_> = self.it.get_mut().iter_mut();
        FilteredColIterMut {
            iter,
            filter: self.filter,
        }
    }
}

// col - iters

/// Immutable iterator over the filtered iterable collection.
pub struct FilteredColIter<'a, I, P>
where
    I: Collection + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: <I::Iterable<'a> as Iterable>::Iter,
    filter: P,
}

impl<'a, I, P> Iterator for FilteredColIter<'a, I, P>
where
    I: Collection,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::Iterable<'a> as Iterable>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            if (self.filter)(x) {
                return Some(x);
            }
        }
    }
}

/// Mutable iterator over the filtered iterable collection.
pub struct FilteredColIterMut<'a, I, P>
where
    I: CollectionMut + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: I::IterMut<'a>,
    filter: P,
}

impl<'a, I, P> Iterator for FilteredColIterMut<'a, I, P>
where
    I: CollectionMut,
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
