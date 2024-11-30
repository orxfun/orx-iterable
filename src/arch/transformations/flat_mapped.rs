use crate::Iterable;
use std::marker::PhantomData;

pub struct FlatMapped<'a, I, U, M>
where
    I: Iterable<'a>,
    U: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    iterable1: I,
    fmap: &'a M,
    phantom: PhantomData<U>,
}

impl<'a, I, U, M> Iterable<'a> for FlatMapped<'a, I, U, M>
where
    I: Iterable<'a>,
    U: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    type Iter = FlatMappedIter<'a, I, U, M>;

    fn iter(&self) -> Self::Iter {
        let mut iter1 = self.iterable1.iter();
        let iterable2: Option<U> = iter1.next().map(self.fmap);
        let iter2: Option<U::Iter> = iterable2.map(|x| x.iter());

        FlatMappedIter {
            fmap: self.fmap,
            iter1,
            iter2,
        }
    }
}

pub struct FlatMappedIter<'a, I, U, M>
where
    I: Iterable<'a>,
    U: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    iter1: I::Iter,
    iter2: Option<U::Iter>,
    fmap: &'a M,
}

impl<'a, I, U, M> Iterator for FlatMappedIter<'a, I, U, M>
where
    I: Iterable<'a>,
    U: Iterable<'a>,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iter2.is_none() {
            return None;
        }

        let iter2 = self.iter2.as_mut().unwrap();
        let value = iter2.next();
        if value.is_some() {
            return value;
        }

        let x = self.iter1.next();
        if x.is_none() {
            return None;
        }

        let iterable2: U = (self.fmap)(x.unwrap());
        let iter2: U::Iter = iterable2.iter();
        self.iter2 = Some(iter2);

        self.next()
    }
}

pub trait IntoFlatMapped<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn flat_mapped<U, M>(self, flat_map: &'a M) -> FlatMapped<'a, Self, U, M>
    where
        M: Fn(Self::Item) -> U,
        U: Iterable<'a>,
    {
        FlatMapped {
            iterable1: self,
            fmap: flat_map,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoFlatMapped<'a> for I where Self: Iterable<'a> {}
