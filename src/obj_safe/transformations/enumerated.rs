use crate::{obj_safe::IterableObj, transformations::Enumerated, Iterable};
use std::boxed::Box;

impl<I> IterableObj for Enumerated<I>
where
    I: Iterable,
{
    type Item = (usize, I::Item);

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().enumerate())
    }
}
