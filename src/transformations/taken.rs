use crate::{Iterable, IterableMut};
use std::marker::PhantomData;

pub struct Taken<'a, I>
where
    I: Iterable<'a>,
{
    take: usize,
    iterable: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> Iterable<'a> for Taken<'a, I>
where
    I: Iterable<'a>,
{
    type Item = <I as Iterable<'a>>::Item;

    type Iter = core::iter::Take<<I as Iterable<'a>>::Iter>;

    fn iter(&self) -> Self::Iter {
        self.iterable.iter().take(self.take)
    }
}

pub trait IntoTaken<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn taken(self, num_items_to_take: usize) -> Taken<'a, Self> {
        Taken {
            iterable: self,
            take: num_items_to_take,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoTaken<'a> for I where I: Iterable<'a> {}

// mut

pub struct TakenMut<'a, I>
where
    I: IterableMut,
{
    take: usize,
    iterable: &'a mut I,
}

impl<'a, I> IterableMut for TakenMut<'a, I>
where
    I: IterableMut,
{
    type ItemMut = I::ItemMut;

    type IterMut<'b> = core::iter::Take<I::IterMut<'b>> where Self: 'b;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.iterable.iter_mut().take(self.take)
    }
}

pub trait IntoTakenMut
where
    Self: IterableMut + Sized,
{
    fn taken_mut(&mut self, num_items_to_take: usize) -> TakenMut<Self> {
        TakenMut {
            iterable: self,
            take: num_items_to_take,
        }
    }
}

impl<I> IntoTakenMut for I where I: IterableMut {}
