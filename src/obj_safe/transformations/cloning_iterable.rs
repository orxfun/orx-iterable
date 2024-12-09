use crate::obj_safe::IterableObj;
use crate::{transformations::CloningIterable, Iterable};
use std::boxed::Box;

impl<I> IterableObj for CloningIterable<I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(<Self as Iterable>::iter(self))
    }
}
