use crate::obj_safe::IterableObj;
use crate::{transformations::Copied, Iterable};
use std::boxed::Box;

impl<'a, T, I> IterableObj for Copied<'a, T, I>
where
    I: Iterable<Item = &'a T>,
    T: Copy + 'a,
{
    type Item = T;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.iter())
    }
}
