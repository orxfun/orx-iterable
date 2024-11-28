use crate::Iterable;

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

// into

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
