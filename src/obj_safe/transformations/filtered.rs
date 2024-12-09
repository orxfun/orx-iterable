use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{Filtered, FilteredCol, FilteredColIter, FilteredColIterMut},
    Collection, CollectionMut, Iterable,
};
use orx_self_or::SoM;
use std::boxed::Box;

impl<I, P> IterableObj for Filtered<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().filter(self.filter))
    }
}

// col

impl<'a, I, E, P> IterableObj for &'a FilteredCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        let iter = self.it.get_ref().iter();
        Box::new(FilteredColIter::<I, P> {
            iter,
            filter: self.filter,
        })
    }
}

impl<I, E, P> CollectionObj for FilteredCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = &Self::Item> + '_> {
        Box::new(self.iter())
    }
}

impl<I, E, P> CollectionMutObj for FilteredCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        let iter: I::IterMut<'_> = self.it.get_mut().iter_mut();
        Box::new(FilteredColIterMut::<I, P> {
            iter,
            filter: self.filter,
        })
    }
}
