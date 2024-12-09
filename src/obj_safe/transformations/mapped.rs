use crate::{obj_safe::IterableObj, transformations::Mapped, Iterable};
use std::boxed::Box;

impl<I, M, U> IterableObj for Mapped<I, M, U>
where
    I: Iterable,
    M: Fn(I::Item) -> U + Copy,
{
    type Item = U;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().map(self.map))
    }
}
