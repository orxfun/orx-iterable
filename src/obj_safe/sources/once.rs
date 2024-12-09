use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    sources::{Once, OnceCol},
};
use std::boxed::Box;

impl<T> IterableObj for Once<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(core::iter::once(self.value.clone()))
    }
}

impl<T> IterableObj for core::iter::Once<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.clone())
    }
}

// col

impl<'a, T> IterableObj for &'a OnceCol<T> {
    type Item = &'a T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(core::iter::once(&self.value))
    }
}

impl<T> CollectionObj for OnceCol<T> {
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(core::iter::once(&self.value))
    }
}

impl<T> CollectionMutObj for OnceCol<T> {
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(core::iter::once(&mut self.value))
    }
}
