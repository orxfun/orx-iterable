use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

/// Wraps an `Iterable` and creates a new `Iterable` which skips the elements
/// of the original iterable that satisfy a given predicate and yields the
/// remaining.
pub struct SkippedWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: I,
    pub(crate) skip_while: P,
}

impl<I, P> Iterable for SkippedWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iter = core::iter::SkipWhile<I::Iter, P>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().skip_while(self.skip_while)
    }
}

// col

/// Wraps an `Collection` and creates a new `Collection` which skips the elements
/// of the original iterable that satisfy a given predicate and yields the
/// remaining.
pub struct SkippedWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) skip_while: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a SkippedWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    type Iter = SkippedWhileColIter<'a, I, P>;

    fn iter(&self) -> Self::Iter {
        let iter = self.it.get_ref().iter();
        SkippedWhileColIter {
            iter,
            skip_while: self.skip_while,
            skipped: false,
        }
    }
}

impl<I, E, P> Collection for SkippedWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iterable<'i>
        = &'i Self
    where
        Self: 'i;

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

impl<I, E, P> CollectionMut for SkippedWhileCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type IterMut<'i>
        = SkippedWhileColIterMut<'i, I, P>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let iter = self.it.get_mut().iter_mut();
        SkippedWhileColIterMut {
            iter,
            skip_while: self.skip_while,
            skipped: false,
        }
    }
}

// col - iters

/// Immutable iterator for skipped while iterable collection.
pub struct SkippedWhileColIter<'a, I, P>
where
    I: Collection + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) iter: <I::Iterable<'a> as Iterable>::Iter,
    pub(crate) skip_while: P,
    pub(crate) skipped: bool,
}

impl<'a, I, P> Iterator for SkippedWhileColIter<'a, I, P>
where
    I: Collection,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::Iterable<'a> as Iterable>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.skipped {
            true => self.iter.next(),
            false => loop {
                match self.iter.next() {
                    Some(x) => match (self.skip_while)(x) {
                        true => {}
                        false => {
                            self.skipped = true;
                            return Some(x);
                        }
                    },
                    None => {
                        self.skipped = true;
                        return None;
                    }
                }
            },
        }
    }
}

/// Mutable iterator for skipped while iterable collection.
pub struct SkippedWhileColIterMut<'a, I, P>
where
    I: CollectionMut + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) iter: I::IterMut<'a>,
    pub(crate) skip_while: P,
    pub(crate) skipped: bool,
}

impl<'a, I, P> Iterator for SkippedWhileColIterMut<'a, I, P>
where
    I: CollectionMut,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::IterMut<'a> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self.skipped {
            true => self.iter.next(),
            false => loop {
                match self.iter.next() {
                    Some(x) => match (self.skip_while)(x) {
                        true => {}
                        false => {
                            self.skipped = true;
                            return Some(x);
                        }
                    },
                    None => {
                        self.skipped = true;
                        return None;
                    }
                }
            },
        }
    }
}
