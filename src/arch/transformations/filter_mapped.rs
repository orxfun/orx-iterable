use crate::Iterable;
use std::marker::PhantomData;

pub struct FilterMapped<'a, I, U, M>
where
    I: Iterable<'a>,
    M: Fn(I::Item) -> Option<U>,
{
    iterable: I,
    filter_map: &'a M,
    phantom: PhantomData<U>,
}

impl<'a, I, U, M> Iterable<'a> for FilterMapped<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    type Item = U;

    type Iter = FilterMappedIter<'a, I, U, M>;

    fn iter(&self) -> Self::Iter {
        FilterMappedIter {
            iter: self.iterable.iter(),
            filter_map: &self.filter_map,
        }
    }
}

pub struct FilterMappedIter<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
    M: Fn(I::Item) -> Option<U>,
{
    iter: I::Iter,
    filter_map: &'a M,
}

impl<'a, I, U, M> Iterator for FilterMappedIter<'a, I, U, M>
where
    I: Iterable<'a> + 'a,
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

pub trait IntoFilterMapped<'a>
where
    Self: Iterable<'a> + Sized,
{
    fn filter_mapped<U, M>(self, filter_map: &'a M) -> FilterMapped<'a, Self, U, M>
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

impl<'a, I> IntoFilterMapped<'a> for I where I: Iterable<'a> {}
