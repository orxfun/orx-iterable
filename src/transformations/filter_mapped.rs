use crate::Iterable;
use std::marker::PhantomData;

pub struct FilterMapped<I, U, M> {
    iterable: I,
    filter_map: M,
    phantom: PhantomData<U>,
}

impl<I, U, M> Iterable for FilterMapped<I, U, M>
where
    I: Iterable,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    type Iter<'a> = FilterMappedIter<'a, I, U, M> where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        FilterMappedIter {
            iter: self.iterable.iter(),
            filter_map: &self.filter_map,
        }
    }
}

pub struct FilterMappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    iter: I::Iter<'a>,
    filter_map: &'a M,
}

impl<'a, I, U, M> Iterator for FilterMappedIter<'a, I, U, M>
where
    I: Iterable + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let x = self.iter.next()?;
            let y = (self.filter_map)(x);
            if y.is_some() {
                return y;
            }
        }
    }
}

pub trait IntoFilterMapped
where
    Self: Iterable + Sized,
{
    fn filter_mapped<U, M>(self, filter_map: M) -> FilterMapped<Self, U, M>
    where
        M: Fn(Self::Item) -> Option<U>,
    {
        FilterMapped {
            iterable: self,
            filter_map,
            phantom: PhantomData,
        }
    }
}

impl<I> IntoFilterMapped for I where I: Iterable {}
