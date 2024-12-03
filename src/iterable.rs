use crate::transformations::{
    Chained, FilterMapped, Filtered, FlatMapped, Flattened, Mapped, MappedWhile, Skipped, Taken,
};

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

    fn filter_mapped<M, U>(self, filter_map: M) -> FilterMapped<Self, M, U>
    where
        M: Fn(Self::Item) -> Option<U> + Copy,
    {
        FilterMapped {
            it: self,
            filter_map,
        }
    }

    fn filtered<P>(self, filter: P) -> Filtered<Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        Filtered { it: self, filter }
    }

    fn flat_mapped<M, U>(self, flat_map: M) -> FlatMapped<Self, M, U>
    where
        U: IntoIterator,
        M: Fn(Self::Item) -> U + Copy,
    {
        FlatMapped { it: self, flat_map }
    }

    fn flattened(self) -> Flattened<Self>
    where
        Self::Item: IntoIterator,
    {
        Flattened { it: self }
    }

    fn mapped_while<M, U>(self, map_while: M) -> MappedWhile<Self, M, U>
    where
        M: Fn(Self::Item) -> Option<U> + Copy,
    {
        MappedWhile {
            it: self,
            map_while,
        }
    }

    fn mapped<M, U>(self, map: M) -> Mapped<Self, M, U>
    where
        M: Fn(Self::Item) -> U + Copy,
    {
        Mapped { it: self, map }
    }

    fn skipped(self, n: usize) -> Skipped<Self> {
        Skipped { it: self, n }
    }

    fn taken(self, n: usize) -> Taken<Self> {
        Taken { it: self, n }
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
