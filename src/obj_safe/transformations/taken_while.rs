use crate::{
    obj_safe::{CollectionMutObj, CollectionObj, IterableObj},
    transformations::{TakenWhile, TakenWhileCol, TakenWhileColIter, TakenWhileColIterMut},
    Collection, CollectionMut, Iterable,
};
use orx_self_or::SoM;
use std::boxed::Box;

impl<I, P> IterableObj for TakenWhile<I, P>
where
    I: Iterable,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        Box::new(self.it.iter().take_while(self.take_while))
    }
}

// col

impl<'a, I, E, P> IterableObj for &'a TakenWhileCol<I, E, P>
where
    I: Collection,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    type Item = &'a I::Item;

    fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
        let iter = self.it.get_ref().iter();
        Box::new(TakenWhileColIter::<I, P> {
            iter,
            filter: self.take_while,
        })
    }
}

impl<I, E, P> CollectionObj for TakenWhileCol<I, E, P>
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

impl<I, E, P> CollectionMutObj for TakenWhileCol<I, E, P>
where
    I: CollectionMut,
    E: SoM<I>,
    P: Fn(&I::Item) -> bool + Copy,
{
    fn boxed_iter_mut(&mut self) -> Box<dyn Iterator<Item = &mut Self::Item> + '_> {
        let iter = self.it.get_mut().iter_mut();
        Box::new(TakenWhileColIterMut::<I, P> {
            iter,
            filter: self.take_while,
        })
    }
}
