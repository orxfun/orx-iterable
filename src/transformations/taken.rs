use crate::{Iterable, IterableMut};

pub struct Taken<I>
where
    I: Iterable,
{
    take: usize,
    iterable: I,
}

impl<I> Iterable for Taken<I>
where
    I: Iterable,
{
    type Item = I::Item;

    type Iter<'a> = core::iter::Take<I::Iter<'a>> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.iterable.iter().take(self.take)
    }
}

pub trait IntoTaken
where
    Self: Iterable + Sized,
{
    fn taken(self, num_items_to_take: usize) -> Taken<Self> {
        Taken {
            iterable: self,
            take: num_items_to_take,
        }
    }
}

impl<I> IntoTaken for I where I: Iterable {}

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
