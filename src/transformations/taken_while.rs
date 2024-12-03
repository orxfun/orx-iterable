use crate::{Iterable, IterableCol};
use orx_exclusive::Exclusive;
use std::marker::PhantomData;

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

    fn it(&self) -> Self::Iter {
        self.it.it().take_while(self.take_while)
    }
}

// col

pub struct TakenWhileCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&&I::Item) -> bool + Copy,
{
    pub(crate) it: E,
    pub(crate) take_while: P,
    pub(crate) phantom: PhantomData<I>,
}

impl<'a, I, E, P> Iterable for &'a TakenWhileCol<I, E, P>
where
    I: IterableCol,
    E: Exclusive<I>,
    P: Fn(&&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    type Iter = core::iter::TakeWhile<<I::Iterable<'a> as Iterable>::Iter, P>;

    fn it(&self) -> Self::Iter {
        self.it.get_ref().iter().take_while(self.take_while)
    }
}

// impl<I, E, P> IterableCol for TakenWhileCol<I, E, P>
// where
//     I: IterableCol,
//     E: Exclusive<I>,
//     P: Fn(&&I::Item) -> bool + Copy,
// {
//     type Item = I::Item;

//     type Iterable<'i> = &'i Self
//     where
//         Self: 'i;

//     type IterMut<'i> = core::iter::TakeWhile<I::IterMut<'i>, P>
//     where
//         Self: 'i;

//     fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
//         todo!()
//     }

//     fn iter_mut(&mut self) -> Self::IterMut<'_> {
//         todo!()
//     }

//     fn as_iterable(&self) -> Self::Iterable<'_> {
//         todo!()
//     }
// }
