use crate::{obj_safe::IterableObj, sources::Repeat};
use std::boxed::Box;

impl<T> IterableObj for Repeat<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(core::iter::repeat(self.value.clone()))
    }
}

impl<T> IterableObj for core::iter::Repeat<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.clone())
    }
}
