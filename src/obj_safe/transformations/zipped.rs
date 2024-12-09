use crate::{obj_safe::IterableObj, transformations::Zipped, Iterable};
use std::boxed::Box;

impl<I1, I2> IterableObj for Zipped<I1, I2>
where
    I1: Iterable,
    I2: Iterable,
{
    type Item = (I1::Item, I2::Item);

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it1.iter().zip(self.it2.iter()))
    }
}
