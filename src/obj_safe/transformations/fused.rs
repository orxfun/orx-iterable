use orx_self_or::SoM;

use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Fused, FusedCol},
    Collection, CollectionMut, Iterable,
};
use std::boxed::Box;

impl<I> IterableObj for Fused<I>
where
    I: Iterable,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().fuse())
    }
}

// col

impl<'a, I, E> IterableObj for &'a FusedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.get_ref().iter().fuse())
    }
}

impl<I, E> CollectionObj for FusedCol<I, E>
where
    I: Collection,
    E: SoM<I>,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I, E> CollectionMutObj for FusedCol<I, E>
where
    I: CollectionMut,
    E: SoM<I>,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(self.it.get_mut().iter_mut().fuse())
    }
}
