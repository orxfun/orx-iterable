use orx_self_or::SoM;

use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Flattened, FlattenedCol},
    Collection, CollectionMut, Iterable,
};
use std::boxed::Box;

impl<I> IterableObj for Flattened<I>
where
    I: Iterable,
    I::Item: IntoIterator,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().flatten())
    }
}

// col

impl<'a, I, E> IterableObj for &'a FlattenedCol<I, E>
where
    I: Collection,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
{
    type Item = &'a <I::Item as IntoIterator>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.get_ref().iter().flatten())
    }
}

impl<I, E> CollectionObj for FlattenedCol<I, E>
where
    I: Collection,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
{
    type Item = <I::Item as IntoIterator>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I, E> CollectionMutObj for FlattenedCol<I, E>
where
    I: CollectionMut,
    I::Item: IntoIterator,
    for<'i> &'i I::Item: IntoIterator<Item = &'i <I::Item as IntoIterator>::Item>,
    for<'i> &'i mut I::Item: IntoIterator<Item = &'i mut <I::Item as IntoIterator>::Item>,
    E: SoM<I>,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(self.it.get_mut().iter_mut().flatten())
    }
}
