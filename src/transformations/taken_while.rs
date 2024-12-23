use crate::{Collection, CollectionMut, Iterable};
use core::marker::PhantomData;
use orx_self_or::SoM;

/// Wraps an `Iterable` and creates a new `Iterable` which yields elements of
/// the original iterable as long as a predicate is satisfied.
pub struct TakenWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: I,
    pub(crate) take_while: P,
}

impl<I, P> Iterable for TakenWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iter = core::iter::TakeWhile<I::Iter, P>;

    fn iter(&self) -> Self::Iter {
        self.it.iter().take_while(self.take_while)
    }
}

// col

/// Wraps an `Collection` and creates a new `Collection` which yields elements of
/// the original iterable as long as a predicate is satisfied.
pub struct TakenWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) take_while: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a TakenWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    type Iter = TakenWhileColIter<'a, I, P>;

    fn iter(&self) -> Self::Iter {
        let iter = self.it.get_ref().iter();
        TakenWhileColIter {
            iter,
            filter: self.take_while,
        }
    }
}

impl<I, E, P> Collection for TakenWhileCol<I, E, P>
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

impl<I, E, P> CollectionMut for TakenWhileCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type IterMut<'i>
        = TakenWhileColIterMut<'i, I, P>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        let iter = self.it.get_mut().iter_mut();
        TakenWhileColIterMut {
            iter,
            filter: self.take_while,
        }
    }
}

// col - iters

/// Immutable iterator for taken while iterable collections.
pub struct TakenWhileColIter<'a, I, P>
where
    I: Collection + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) iter: <I::Iterable<'a> as Iterable>::Iter,
    pub(crate) filter: P,
}

impl<'a, I, P> Iterator for TakenWhileColIter<'a, I, P>
where
    I: Collection,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::Iterable<'a> as Iterable>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        (self.filter)(x).then_some(x)
    }
}

/// Mutable iterator for taken while iterable collections.
pub struct TakenWhileColIterMut<'a, I, P>
where
    I: CollectionMut + 'a,
    P: Fn(&I::Item) -> bool + Copy,
{
    pub(crate) iter: I::IterMut<'a>,
    pub(crate) filter: P,
}

impl<'a, I, P> Iterator for TakenWhileColIterMut<'a, I, P>
where
    I: CollectionMut,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = <I::IterMut<'a> as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.iter.next()?;
        (self.filter)(x).then_some(x)
    }
}
