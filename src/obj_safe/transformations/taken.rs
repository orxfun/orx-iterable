use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Taken, TakenCol},
    Collection, CollectionMut, Iterable,
};
use orx_self_or::SoM;
use std::boxed::Box;

impl<I> IterableObj for Taken<I>
where
    I: Iterable,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().take(self.n))
    }
}

// col

impl<'a, I, E> IterableObj for &'a TakenCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.get_ref().iter().take(self.n))
    }
}

impl<I, E> CollectionObj for TakenCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I, E> CollectionMutObj for TakenCol<I, E>
where
    I: CollectionMut,
    E: SoM<I>,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(self.it.get_mut().iter_mut().take(self.n))
    }
}
