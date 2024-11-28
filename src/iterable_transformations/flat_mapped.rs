use crate::Iterable;
use std::marker::PhantomData;

// iterable

pub struct FlatMapped<I, T, U, M>
where
    M: Fn(T) -> U,
    U: Iterable,
{
    it1: I,
    fmap: M,
    phantom: PhantomData<T>,
}

impl<I, T, U, M> Iterable for FlatMapped<I, T, U, M>
where
    I: Iterable<Item = T>,
    M: Fn(T) -> U,
    U: Iterable,
{
    type Item = U::Item;

    type Iter<'a> = FlatMappedIter<'a, I, T, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let iter1 = self.it1.iter();
        FlatMappedIter::new(iter1, &self.fmap)
    }
}

// iter

pub struct FlatMappedIter<'a, I, T, U, M>
where
    I: Iterable<Item = T> + 'a,
    M: Fn(T) -> U,
    U: Iterable + 'a,
{
    iter1: I::Iter<'a>,
    iter2: Option<<U as Iterable>::Iter<'a>>,
    fmap: &'a M,
}

impl<'a, I, T, U, M> FlatMappedIter<'a, I, T, U, M>
where
    I: Iterable<Item = T> + 'a,
    M: Fn(T) -> U,
    U: Iterable,
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
                let iterable2 = unsafe { into_ref(&fmap(iterable2)) };
                Some(iterable2.iter())
            }
            None => None,
        }
    }
}

impl<'a, I, T, U, M> Iterator for FlatMappedIter<'a, I, T, U, M>
where
    I: Iterable<Item = T> + 'a,
    M: Fn(T) -> U,
    U: Iterable,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter2 {
            Some(it2) => match it2.next() {
                Some(x) => Some(x),
                None => {
                    self.iter2 = Self::next_iter2(&mut self.iter1, self.fmap);
                    self.next()
                }
            },
            None => None,
        }
    }
}

// into

pub trait IntoFlatMapped<T>
where
    Self: Iterable<Item = T>,
{
    fn flat_mapped<U, M>(self, flat_map: M) -> FlatMapped<Self, T, U, M>
    where
        M: Fn(T) -> U,
        U: Iterable,
        Self: Sized,
    {
        FlatMapped {
            it1: self,
            fmap: flat_map,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoFlatMapped<T> for I where Self: Iterable<Item = T> {}
