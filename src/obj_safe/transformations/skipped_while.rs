use orx_self_or::SoM;

use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{SkippedWhile, SkippedWhileCol, SkippedWhileColIter, SkippedWhileColIterMut},
    Collection, CollectionMut, Iterable,
};
use std::boxed::Box;

impl<I, P> IterableObj for SkippedWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().skip_while(self.skip_while))
    }
}

// col

impl<'a, I, E, P> IterableObj for &'a SkippedWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        let iter = self.it.get_ref().iter();
        Box::new(SkippedWhileColIter::<I, P> {
            iter,
            skip_while: self.skip_while,
            skipped: false,
        })
    }
}

impl<I, E, P> CollectionObj for SkippedWhileCol<I, E, P>
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

impl<I, E, P> CollectionMutObj for SkippedWhileCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        let iter = self.it.get_mut().iter_mut();
        Box::new(SkippedWhileColIterMut::<I, P> {
            iter,
            skip_while: self.skip_while,
            skipped: false,
        })
    }
}
