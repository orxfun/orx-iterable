use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    sources::{Empty, EmptyCol},
    Collection, CollectionMut, Iterable,
};
use std::boxed::Box;

impl<T> IterableObj for Empty<T> {
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<T> IterableObj for core::iter::Empty<T> {
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}

// col

impl<'a, T> IterableObj for &'a EmptyCol<T> {
    type Item = &'a T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<T> CollectionObj for EmptyCol<T> {
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<T> CollectionMutObj for EmptyCol<T> {
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(self.iter_mut())
    }
}
