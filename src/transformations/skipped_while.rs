use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use core::marker::PhantomData;

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

pub struct SkippedWhileCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) skip_while: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a SkippedWhileCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
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

impl<I, E, P> IterableCol for SkippedWhileCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iterable<'i> = &'i Self
    where
        Self: 'i;

    type IterMut<'i> = SkippedWhileColIterMut<'i, I, P>
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

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}

// col - iters

pub struct SkippedWhileColIter<'a, I, P>
where
    I: IterableCol + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: <I::Iterable<'a> as Iterable>::Iter,
    skip_while: P,
    skipped: bool,
}

impl<'a, I, P> Iterator for SkippedWhileColIter<'a, I, P>
where
    I: IterableCol,
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

pub struct SkippedWhileColIterMut<'a, I, P>
where
    I: IterableCol + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    iter: I::IterMut<'a>,
    skip_while: P,
    skipped: bool,
}

impl<'a, I, P> Iterator for SkippedWhileColIterMut<'a, I, P>
where
    I: IterableCol,
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
