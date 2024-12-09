use crate::{
    obj_safe::IterableObj,
    transformations::{FlatMapped, FlatMappedIter},
    Iterable,
};
use std::boxed::Box;

impl<I, M, U> IterableObj for FlatMapped<I, M, U>
where
    I: Iterable,
    U: IntoIterator,
    M: Fn(I::Item) -> U + Copy,
{
    type Item = U::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        let mut iter1 = self.it.iter();
        let iterable2: Option<U> = iter1.next().map(self.flat_map);
        let iter2: Option<U::IntoIter> = iterable2.map(|x| x.into_iter());

        Box::new(FlatMappedIter::<I, M, U> {
            flat_map: self.flat_map,
            iter1,
            iter2,
        })
    }
}
