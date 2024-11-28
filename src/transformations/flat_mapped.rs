use crate::{Iterable, IterableOnce};
use std::marker::PhantomData;

pub struct FlatMapped<I, U, M> {
    it1: I,
    fmap: M,
    phantom: PhantomData<U>,
}

impl<I, U, M> Iterable for FlatMapped<I, U, M>
where
    I: Iterable,
    U: Iterable,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    type Iter<'a> = FlatMappedIter<'a, I, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let iter1 = self.it1.iter();
        FlatMappedIter::new(iter1, &self.fmap)
    }
}

pub struct FlatMappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    U: Iterable + 'a,
    M: Fn(I::Item) -> U,
{
    iter1: I::Iter<'a>,
    iter2: Option<<U as Iterable>::Iter<'a>>,
    fmap: &'a M,
}

impl<'a, I, U, M> FlatMappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    U: Iterable + 'a,
    M: Fn(I::Item) -> U,
{
    fn new(mut iter1: I::Iter<'a>, fmap: &'a M) -> Self {
        let iter2 = Self::next_iter2(&mut iter1, fmap);
        Self { iter1, iter2, fmap }
    }

    fn next_iter2(iter1: &mut I::Iter<'a>, fmap: &'a M) -> Option<<U as Iterable>::Iter<'a>> {
        unsafe fn into_ref<'b, U>(reference: &U) -> &'b U {
            unsafe { &*(reference as *const U) }
        }

        match iter1.next() {
            Some(iterable2) => {
                let iterable2 = std::hint::black_box(unsafe { into_ref(&fmap(iterable2)) });
                Some(iterable2.iter())
            }
            None => None,
        }
    }
}

impl<'a, I, U, M> Iterator for FlatMappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    U: Iterable + 'a,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.iter2 {
                Some(it2) => match it2.next() {
                    Some(x) => return Some(x),
                    None => {
                        self.iter2 = Self::next_iter2(&mut self.iter1, self.fmap);
                    }
                },
                None => return None,
            }
        }
    }
}

pub trait IntoFlatMapped
where
    Self: Iterable + Sized,
{
    fn flat_mapped<U, M>(self, flat_map: M) -> FlatMapped<Self, U, M>
    where
        M: Fn(Self::Item) -> U,
        U: Iterable,
    {
        FlatMapped {
            it1: self,
            fmap: flat_map,
            phantom: PhantomData,
        }
    }
}

impl<I> IntoFlatMapped for I where Self: Iterable {}

// once

impl<I, U, M> IterableOnce for FlatMapped<I, U, M>
where
    I: IterableOnce,
    U: IterableOnce,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    type Iter = FlatMappedIterOnce<I, U, M>;

    fn it_once(self) -> Self::Iter {
        let iter1 = self.it1.it_once();
        FlatMappedIterOnce::new(iter1, self.fmap)
    }
}

pub struct FlatMappedIterOnce<I, U, M>
where
    I: IterableOnce,
    U: IterableOnce,
    M: Fn(I::Item) -> U,
{
    iter1: I::Iter,
    iter2: Option<<U as IterableOnce>::Iter>,
    fmap: M,
}

impl<I, U, M> FlatMappedIterOnce<I, U, M>
where
    I: IterableOnce,
    U: IterableOnce,
    M: Fn(I::Item) -> U,
{
    fn new(mut iter1: I::Iter, fmap: M) -> Self {
        let iter2 = Self::next_iter2(&mut iter1, &fmap);
        Self { iter1, iter2, fmap }
    }

    fn next_iter2(iter1: &mut I::Iter, fmap: &M) -> Option<<U as IterableOnce>::Iter> {
        match iter1.next() {
            Some(iterable2) => {
                let iterable2 = fmap(iterable2);
                Some(iterable2.it_once())
            }
            None => None,
        }
    }
}

impl<I, U, M> Iterator for FlatMappedIterOnce<I, U, M>
where
    I: IterableOnce,
    U: IterableOnce,
    M: Fn(I::Item) -> U,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.iter2 {
                Some(it2) => match it2.next() {
                    Some(x) => return Some(x),
                    None => {
                        self.iter2 = Self::next_iter2(&mut self.iter1, &self.fmap);
                    }
                },
                None => return None,
            }
        }
    }
}

pub trait IntoFlatMappedOnce
where
    Self: IterableOnce + Sized,
{
    fn flat_mapped_once<U, M>(self, flat_map: M) -> FlatMapped<Self, U, M>
    where
        M: Fn(Self::Item) -> U,
        U: IterableOnce,
    {
        FlatMapped {
            it1: self,
            fmap: flat_map,
            phantom: PhantomData,
        }
    }
}

impl<I> IntoFlatMappedOnce for I where Self: IterableOnce {}
