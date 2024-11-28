use crate::{Iterable, IterableMut, IterableOnce};

pub struct Flattened<I> {
    it1: I,
}

impl<I> Iterable for Flattened<I>
where
    I: Iterable,
    I::Item: Iterable,
{
    type Item = <I::Item as Iterable>::Item;

    type Iter<'a> = FlattenedIter<'a, I> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let iter1 = self.it1.iter();
        FlattenedIter::new(iter1)
    }
}

pub struct FlattenedIter<'a, I>
where
    I: Iterable + 'a,
    I::Item: Iterable,
{
    iter1: I::Iter<'a>,
    iter2: Option<<I::Item as Iterable>::Iter<'a>>,
}

impl<'a, I> FlattenedIter<'a, I>
where
    I: Iterable,
    I::Item: Iterable,
{
    fn new(mut iter1: I::Iter<'a>) -> Self {
        let iter2 = Self::next_iter2(&mut iter1);
        Self { iter1, iter2 }
    }

    fn next_iter2(iter1: &mut I::Iter<'a>) -> Option<<I::Item as Iterable>::Iter<'a>> {
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
    I: Iterable,
    I::Item: Iterable,
{
    type Item = <I::Item as Iterable>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter2 {
            Some(it2) => match it2.next() {
                Some(x) => Some(x),
                None => {
                    self.iter2 = Self::next_iter2(&mut self.iter1);
                    self.next()
                }
            },
            None => None,
        }
    }
}

pub trait IntoFlattened
where
    Self: Iterable,
    Self::Item: Iterable,
{
    fn flattened(self) -> Flattened<Self>
    where
        Self: Sized,
    {
        Flattened { it1: self }
    }
}

impl<I> IntoFlattened for I
where
    I: Iterable,
    I::Item: Iterable,
{
}

// once

impl<I> IterableOnce for Flattened<I>
where
    I: IterableOnce,
    I::Item: IterableOnce,
{
    type Item = <I::Item as IterableOnce>::Item;

    type Iter = FlattenedIterOnce<I>;

    fn it_once(self) -> Self::Iter {
        let iter1 = self.it1.it_once();
        FlattenedIterOnce::new(iter1)
    }
}

pub struct FlattenedIterOnce<I>
where
    I: IterableOnce,
    I::Item: IterableOnce,
{
    iter1: I::Iter,
    iter2: Option<<I::Item as IterableOnce>::Iter>,
}

impl<I> FlattenedIterOnce<I>
where
    I: IterableOnce,
    I::Item: IterableOnce,
{
    fn new(mut iter1: I::Iter) -> Self {
        let iter2 = Self::next_iter2(&mut iter1);
        Self { iter1, iter2 }
    }

    fn next_iter2(iter1: &mut I::Iter) -> Option<<I::Item as IterableOnce>::Iter> {
        let iterable2 = iter1.next()?;
        Some(iterable2.it_once())
    }
}

impl<I> Iterator for FlattenedIterOnce<I>
where
    I: IterableOnce,
    I::Item: IterableOnce,
{
    type Item = <I::Item as IterableOnce>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter2 {
            Some(it2) => match it2.next() {
                Some(x) => Some(x),
                None => {
                    self.iter2 = Self::next_iter2(&mut self.iter1);
                    self.next()
                }
            },
            None => None,
        }
    }
}

pub trait IntoFlattenedOnce
where
    Self: IterableOnce,
    Self::Item: IterableOnce,
{
    fn flattened_once(self) -> Flattened<Self>
    where
        Self: Sized,
    {
        Flattened { it1: self }
    }
}

impl<I> IntoFlattenedOnce for I
where
    I: IterableOnce,
    I::Item: IterableOnce,
{
}

// mut

pub struct FlattenedMut<'a, I> {
    it1: &'a mut I,
}

impl<'a, I> IterableMut for FlattenedMut<'a, I>
where
    I: IterableMut + 'a,
    I::ItemMut: IterableMut,
{
    type ItemMut = <I::ItemMut as IterableMut>::ItemMut;

    type IterMut<'b> = FlattenedMutIter<'b, I> where Self: 'b;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        FlattenedMutIter::new(self.it1.xyz())
    }
}

pub struct FlattenedMutIter<'a, I>
where
    I: IterableMut + 'a,
    I::ItemMut: IterableMut,
{
    iter1: I::IterMut<'a>,
    iter2: Option<<I::ItemMut as IterableMut>::IterMut<'a>>,
}

impl<'a, I> FlattenedMutIter<'a, I>
where
    I: IterableMut + 'a,
    I::ItemMut: IterableMut,
{
    fn new(mut iter1: I::IterMut<'a>) -> Self {
        let iter2 = Self::next_iter2(&mut iter1);
        Self { iter1, iter2 }
    }

    fn next_iter2(iter1: &mut I::IterMut<'a>) -> Option<<I::ItemMut as IterableMut>::IterMut<'a>> {
        unsafe fn into_mut<'b, U>(reference: &mut U) -> &'b mut U {
            unsafe { &mut *(reference as *mut U) }
        }

        match iter1.next() {
            Some(iterable2) => {
                let iterable2 = unsafe { into_mut(iterable2) };
                Some(iterable2.xyz())
            }
            None => None,
        }
    }
}

impl<'a, I> Iterator for FlattenedMutIter<'a, I>
where
    I: IterableMut + 'a,
    I::ItemMut: IterableMut,
{
    type Item = &'a mut <I::ItemMut as IterableMut>::ItemMut;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.iter2 {
            Some(it2) => match it2.next() {
                Some(x) => Some(x),
                None => {
                    self.iter2 = Self::next_iter2(&mut self.iter1);
                    self.next()
                }
            },
            None => None,
        }
    }
}

pub trait IntoFlattenedMut
where
    Self: IterableMut + Sized,
    Self::ItemMut: IterableMut,
{
    fn flattened_mut(&mut self) -> FlattenedMut<Self> {
        FlattenedMut { it1: self }
    }
}

impl<I> IntoFlattenedMut for I
where
    I: IterableMut,
    I::ItemMut: IterableMut,
{
}
