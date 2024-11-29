use crate::{Iterable, IterableMut};
use std::marker::PhantomData;

pub struct Flattened<'a, I>
where
    I: Iterable<'a>,
    I::Item: Iterable<'a>,
{
    it1: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> Iterable<'a> for Flattened<'a, I>
where
    I: Iterable<'a> + 'a,
    I::Item: Iterable<'a>,
{
    type Item = <I::Item as Iterable<'a>>::Item;

    type Iter = FlattenedIter<'a, I>;

    fn iter(&self) -> Self::Iter {
        let iter1 = self.it1.iter();
        FlattenedIter::new(iter1)
    }
}

pub struct FlattenedIter<'a, I>
where
    I: Iterable<'a> + 'a,
    I::Item: Iterable<'a>,
{
    iter1: I::Iter,
    iter2: Option<<I::Item as Iterable<'a>>::Iter>,
}

impl<'a, I> FlattenedIter<'a, I>
where
    I: Iterable<'a>,
    I::Item: Iterable<'a>,
{
    fn new(mut iter1: I::Iter) -> Self {
        let iter2 = Self::next_iter2(&mut iter1);
        Self { iter1, iter2 }
    }

    fn next_iter2(iter1: &mut I::Iter) -> Option<<I::Item as Iterable<'a>>::Iter> {
        unsafe fn into_ref<'b, U>(reference: &U) -> &'b U {
            unsafe { &*(reference as *const U) }
        }

        match iter1.next() {
            Some(iterable2) => {
                let iterable2 = unsafe { into_ref(&iterable2) };
                Some(iterable2.iter())
            }
            None => None,
        }
    }
}

impl<'a, I> Iterator for FlattenedIter<'a, I>
where
    I: Iterable<'a>,
    I::Item: Iterable<'a>,
{
    type Item = <I::Item as Iterable<'a>>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.iter2 {
                Some(it2) => {
                    let x = it2.next();
                    if x.is_some() {
                        return x;
                    }
                    self.iter2 = Self::next_iter2(&mut self.iter1);
                }
                None => return None,
            }
        }
    }
}

pub trait IntoFlattened<'a>
where
    Self: Iterable<'a>,
    Self::Item: Iterable<'a>,
{
    fn flattened(self) -> Flattened<'a, Self>
    where
        Self: Sized,
    {
        Flattened {
            it1: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoFlattened<'a> for I
where
    I: Iterable<'a>,
    I::Item: Iterable<'a>,
{
}

// mut

pub struct FlattenedMut<'a, I>
where
    I: IterableMut<'a> + 'a,
    I::ItemMut: IterableMut<'a>,
{
    it1: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> IterableMut<'a> for FlattenedMut<'a, I>
where
    I: IterableMut<'a> + 'a,
    I::ItemMut: IterableMut<'a>,
{
    type ItemMut = <I::ItemMut as IterableMut<'a>>::ItemMut;

    type IterMut = FlattenedMutIter<'a, I>;

    fn iter_mut(&'a mut self) -> Self::IterMut {
        FlattenedMutIter::new(self.it1.iter_mut())
    }
}

pub struct FlattenedMutIter<'a, I>
where
    I: IterableMut<'a> + 'a,
    I::ItemMut: IterableMut<'a>,
{
    iter1: I::IterMut,
    iter2: Option<<I::ItemMut as IterableMut<'a>>::IterMut>,
}

impl<'a, I> FlattenedMutIter<'a, I>
where
    I: IterableMut<'a> + 'a,
    I::ItemMut: IterableMut<'a>,
{
    fn new(mut iter1: I::IterMut) -> Self {
        let iter2 = Self::next_iter2(&mut iter1);
        Self { iter1, iter2 }
    }

    fn next_iter2(iter1: &mut I::IterMut) -> Option<<I::ItemMut as IterableMut<'a>>::IterMut> {
        unsafe fn into_mut<'b, U>(reference: &mut U) -> &'b mut U {
            unsafe { &mut *(reference as *mut U) }
        }

        match iter1.next() {
            Some(mut iterable2) => {
                let iterable2 = unsafe { into_mut(&mut iterable2) };
                Some(iterable2.iter_mut())
            }
            None => None,
        }
    }
}

impl<'a, I> Iterator for FlattenedMutIter<'a, I>
where
    I: IterableMut<'a> + 'a,
    I::ItemMut: IterableMut<'a>,
{
    type Item = <I::ItemMut as IterableMut<'a>>::ItemMut;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match &mut self.iter2 {
                Some(it2) => {
                    let x = it2.next();
                    if x.is_some() {
                        return x;
                    }
                    self.iter2 = Self::next_iter2(&mut self.iter1);
                }
                None => return None,
            }
        }
    }
}

pub trait IntoFlattenedMut<'a>
where
    Self: IterableMut<'a> + Sized,
    Self::ItemMut: IterableMut<'a>,
{
    fn flattened_mut(self) -> FlattenedMut<'a, Self> {
        FlattenedMut {
            it1: self,
            phantom: PhantomData,
        }
    }
}

impl<'a, I> IntoFlattenedMut<'a> for I
where
    I: IterableMut<'a>,
    I::ItemMut: IterableMut<'a>,
{
}
