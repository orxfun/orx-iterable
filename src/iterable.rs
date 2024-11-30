use crate::transformations::{Chained, Filtered, Mapped};

pub trait Iterable: Sized {
    type Item;

    type Iter: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::Iter;

    // provided

    fn chained<I>(self, other: I) -> Chained<Self, I>
    where
        I: Iterable<Item = Self::Item>,
    {
        Chained {
            it1: self,
            it2: other,
        }
    }

    fn mapped<M, U>(self, map: M) -> Mapped<Self, M, U>
    where
        M: Fn(Self::Item) -> U + Copy,
    {
        Mapped {
            iterable: self,
            map,
        }
    }

    fn filtered<P>(self, filter: P) -> Filtered<Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        Filtered { it: self, filter }
    }
}

// impl

impl<'a, X> Iterable for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    type Iter = <&'a X as IntoIterator>::IntoIter;

    fn iter(&self) -> Self::Iter {
        self.into_iter()
    }
}
