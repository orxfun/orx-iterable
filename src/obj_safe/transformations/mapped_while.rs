use crate::{obj_safe::IterableObj, transformations::MappedWhile, Iterable};
use std::boxed::Box;

impl<I, M, U> IterableObj for MappedWhile<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U> + Copy,
{
    type Item = U;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().map_while(self.map_while))
    }
}
