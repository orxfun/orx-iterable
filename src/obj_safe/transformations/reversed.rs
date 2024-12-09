use orx_self_or::SoM;

use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Reversed, ReversedCol},
    Collection, CollectionMut, Iterable,
};
use std::boxed::Box;

impl<I> IterableObj for Reversed<I>
where
    I: Iterable,
    I::Iter: DoubleEndedIterator,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().rev())
    }
}

// col

impl<'a, I, E> IterableObj for &'a ReversedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.get_ref().iter().rev())
    }
}

impl<I, E> CollectionObj for ReversedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I, E> CollectionMutObj for ReversedCol<I, E>
where
    I: CollectionMut,
    E: SoM<I>,
    for<'b> <I::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
    for<'b> I::IterMut<'b>: DoubleEndedIterator,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(self.it.get_mut().iter_mut().rev())
    }
}
