use crate::{Iterable, IterableMut};

pub struct TakenWhile<'a, I, P>
where
    I: Iterable<'a>,
    P: Fn(&I::Item) -> bool,
{
    iterable: I,
    take_while: &'a P,
}

impl<'a, I, P> Iterable<'a> for TakenWhile<'a, I, P>
where
    I: Iterable<'a>,
    P: Fn(&I::Item) -> bool,
{
    type Item = I::Item;

    type Iter = core::iter::TakeWhile<I::Iter, &'a P>;

    fn iter(&self) -> Self::Iter {
        self.iterable.iter().take_while(&self.take_while)
    }
}

pub trait IntoTakenWhile<'a>
where
    Self: Sized + Iterable<'a>,
{
    fn taken_while<P>(self, take_while_predicate: &'a P) -> TakenWhile<'a, Self, P>
    where
        P: Fn(&Self::Item) -> bool,
    {
        TakenWhile {
            iterable: self,
            take_while: take_while_predicate,
        }
    }
}

impl<'a, I> IntoTakenWhile<'a> for I where I: Iterable<'a> {}

// mut

pub struct TakenWhileMut<'a, I, F>
where
    I: IterableMut,
    F: Fn(&I::ItemMut) -> bool,
{
    iterable: &'a mut I,
    take_while: &'a F,
}

impl<'a, I, F> IterableMut for TakenWhileMut<'a, I, F>
where
    I: IterableMut + 'a,
    F: Fn(&I::ItemMut) -> bool,
{
    type ItemMut = I::ItemMut;

    type IterMut<'b> = TakenWhileMutIter<'b, I, F> where Self: 'b;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        TakenWhileMutIter {
            iter: self.iterable.iter_mut(),
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
    fn taken_while_mut<'a, F>(&'a mut self, take_while: &'a F) -> TakenWhileMut<'a, Self, F>
    where
        F: Fn(&Self::ItemMut) -> bool,
    {
        TakenWhileMut {
            iterable: self,
            take_while,
        }
    }
}

impl<'a, I> IntoTakenWhileMut for I where I: IterableMut {}
