use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Chained, ChainedCol},
    Collection, CollectionMut, Iterable,
};
use orx_self_or::SoM;
use std::boxed::Box;

impl<I1, I2> IterableObj for Chained<I1, I2>
where
    I1: Iterable,
    I2: Iterable<Item = I1::Item>,
{
    type Item = I1::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it1.iter().chain(self.it2.iter()))
    }
}

// col

impl<'a, I1, I2, E1, E2> IterableObj for &'a ChainedCol<I1, I2, E1, E2>
where
    I1: Collection,
    I2: Collection<Item = <I1 as Collection>::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    type Item = &'a <I1 as Collection>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it1.get_ref().iter().chain(self.it2.get_ref().iter()))
    }
}

impl<I1, I2, E1, E2> CollectionObj for ChainedCol<I1, I2, E1, E2>
where
    I1: Collection,
    I2: Collection<Item = <I1 as Collection>::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    type Item = <I1 as Collection>::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I1, I2, E1, E2> CollectionMutObj for ChainedCol<I1, I2, E1, E2>
where
    I1: CollectionMut,
    I2: CollectionMut<Item = <I1 as Collection>::Item>,
    E1: SoM<I1>,
    E2: SoM<I2>,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        Box::new(
            self.it1
                .get_mut()
                .iter_mut()
                .chain(self.it2.get_mut().iter_mut()),
        )
    }
}
