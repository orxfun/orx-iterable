use crate::{Iterable, IterableMut};

pub struct TakenWhile<I, P> {
    iterable: I,
    take_while: P,
}

impl<I, P> Iterable for TakenWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter<'a> = core::iter::TakeWhile<I::Iter<'a>, &'a P> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().take_while(&self.take_while)
    }
}

pub trait IntoTakenWhile
where
    Self: Sized + Iterable,
{
    fn taken_while<P>(self, take_while_predicate: P) -> TakenWhile<Self, P>
    where
        P: Fn(&Self::Item) -> bool,
    {
        TakenWhile {
            iterable: self,
            take_while: take_while_predicate,
        }
    }
}

impl<I> IntoTakenWhile for I where I: Iterable {}

// mut

pub struct TakenWhileMut<'a, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    iterable: &'a mut I,
    take_while: F,
}

impl<'a, I, F> IterableMut for TakenWhileMut<'a, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    type ItemMut = I::ItemMut;

    type IterMut<'b> = TakenWhileMutIter<'b, I, F> where Self: 'b;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        TakenWhileMutIter {
            iter: self.iterable.xyz(),
            take_while: &self.take_while,
        }
    }
}

pub struct TakenWhileMutIter<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    iter: I::IterMut<'a>,
    take_while: &'a F,
}

impl<'a, I, F> Iterator for TakenWhileMutIter<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    type Item = &'a mut I::ItemMut;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(x) => match (self.take_while)(&x) {
                true => Some(x),
                false => None,
            },
            None => None,
        }
    }
}

pub trait IntoTakenWhileMut
where
    Self: Sized + IterableMut,
{
    fn taken_while_mut<F>(&mut self, take_while: F) -> TakenWhileMut<Self, F>
    where
        F: Fn(&Self::ItemMut) -> bool,
    {
        TakenWhileMut {
            iterable: self,
            take_while,
        }
    }
}

impl<I> IntoTakenWhileMut for I where I: IterableMut {}
