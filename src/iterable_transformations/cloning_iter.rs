use crate::{Iterable, IterableOnce};

pub struct CloningIterable<I>(I)
where
    I: Iterator + Clone;

impl<I: Iterator + Clone> CloningIterable<I>
where
    I: Iterator + Clone,
{
    pub fn new(iter: I) -> Self {
        Self(iter)
    }
}

impl<I> IterableOnce for CloningIterable<I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.0
    }
}

impl<I> Iterable for CloningIterable<I>
where
    I: Iterator + Clone,
{
    type Item = I::Item;

    type Iter<'a> = I where Self: 'a;

    fn iter(&self) -> Self::Iter<'_> {
        self.0.clone()
    }
}

// into

pub trait IntoCloningIterable: IntoIterator
where
    <Self as IntoIterator>::IntoIter: Clone,
{
    fn into_iterable(self) -> CloningIterable<<Self as IntoIterator>::IntoIter>;
}

impl<I> IntoCloningIterable for I
where
    I: IntoIterator,
    I::IntoIter: Clone,
{
    fn into_iterable(self) -> CloningIterable<<Self as IntoIterator>::IntoIter> {
        CloningIterable(self.into_iter())
    }
}
