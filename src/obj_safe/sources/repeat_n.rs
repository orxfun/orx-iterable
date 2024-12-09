use crate::{obj_safe::IterableObj, sources::RepeatN};
use std::boxed::Box;

impl<T> IterableObj for RepeatN<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(core::iter::repeat_n(self.value.clone(), self.count))
    }
}

impl<T> IterableObj for core::iter::RepeatN<T>
where
    T: Clone,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.clone())
    }
}
