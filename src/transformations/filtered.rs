use crate::{exclusive::Exclusive, Iterable, IterableCol};
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

    fn iter(&self) -> Self::Iter {
        self.it.iter().filter(self.filter)
    }
}

// col

pub struct FilteredCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&&I::Item) -> bool + Copy,
{
    pub(crate) it1: E,
    pub(crate) filter: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<I, E, P> IterableCol for FilteredCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    type Iter<'i> = core::iter::Filter<I::Iter<'i>, P>
    where
        Self: 'i;

    type IterMut<'i> = core::iter::Empty<&'i mut Self::Item>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        let mut x = self.it1.get_ref().iter();
        let y: &I::Item = x.next().unwrap();

        let mut x = self.it1.get_ref().iter().filter(self.filter);
        // let y: &I::Item = x.next().unwrap();

        // let mut x = self.it1.get_ref().iter().filter(|x| (self.filter)(x));

        x
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        todo!()
    }
}
