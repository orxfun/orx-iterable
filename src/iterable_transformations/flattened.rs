use crate::Iterable;
use std::marker::PhantomData;

// iterable

pub struct Flattened<I, T> {
    it1: I,
    phantom: PhantomData<T>,
}

// impl<I, T> IterableOnce for Flattened<I, T>
// where
//     I: IterableOnce,
//     I::Item: IterableOnce<Item = T>,
// {
//     type Item = T;

//     fn it_once(self) -> impl Iterator<Item = Self::Item> {
//         self.it1.it_once().flat_map(|it2| it2.it_once())
//     }
// }

impl<I, T> Iterable for Flattened<I, T>
where
    I: Iterable,
    I::Item: Iterable<Item = T>,
{
    type Item = T;

    type Iter<'a> = FlattenedIter<'a, I, T> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        let iter1 = self.it1.iter();
        FlattenedIter::new(iter1)
    }
}

pub struct FlattenedIter<'a, I, T>
where
    I: Iterable + 'a,
    I::Item: Iterable<Item = T>,
{
    iter1: I::Iter<'a>,
    iter2: Option<<I::Item as Iterable>::Iter<'a>>,
}

impl<'a, I, T> FlattenedIter<'a, I, T>
where
    I: Iterable,
    I::Item: Iterable<Item = T>,
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

impl<'a, I, T> Iterator for FlattenedIter<'a, I, T>
where
    I: Iterable,
    I::Item: Iterable<Item = T>,
{
    type Item = T;

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

// into

pub trait IntoFlattened<T>
where
    Self: Iterable,
    Self::Item: Iterable<Item = T>,
{
    fn flattened(self) -> Flattened<Self, T>
    where
        Self: Sized,
    {
        Flattened {
            it1: self,
            phantom: PhantomData,
        }
    }
}

impl<T, I> IntoFlattened<T> for I
where
    I: Iterable,
    I::Item: Iterable<Item = T>,
{
}
