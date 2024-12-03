use crate::{
    transformations::{ChainedCol, FilteredCol, FlattenedCol, SkippedCol, TakenCol},
    Iterable,
};

// TODO: consider IterableCol: Iterable bound

pub trait IterableCol: Sized // for<'a> &'a Self: Iterable<Item = &'a Self::Item>,
{
    type Item;

    type Iter<'i>: Iterator<Item = &'i Self::Item>
    where
        Self: 'i;

    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    // provided exclusive

    fn into_chained<I>(self, other: I) -> ChainedCol<Self, I, Self, I>
    where
        I: IterableCol<Item = Self::Item>,
        for<'a> &'a I: Iterable<Item = &'a Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

    fn chained_mut<'a, I>(
        &'a mut self,
        other: &'a mut I,
    ) -> ChainedCol<Self, I, &'a mut Self, &'a mut I>
    where
        I: IterableCol<Item = Self::Item>,
        for<'b> &'b I: Iterable<Item = &'b Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

    fn into_filtered<P>(self, filter: P) -> FilteredCol<Self, Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        FilteredCol {
            it: self,
            filter,
            phantom: Default::default(),
        }
    }

    fn filtered_mut<P>(&mut self, filter: P) -> FilteredCol<Self, &mut Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        FilteredCol {
            it: self,
            filter,
            phantom: Default::default(),
        }
    }

    fn into_flattened(self) -> FlattenedCol<Self, Self>
    where
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
        for<'i> &'i mut Self::Item: IntoIterator<Item = &'i mut <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    fn flattened_mut(&mut self) -> FlattenedCol<Self, &mut Self>
    where
        Self::Item: IntoIterator,
        for<'i> &'i Self::Item: IntoIterator<Item = &'i <Self::Item as IntoIterator>::Item>,
        for<'i> &'i mut Self::Item: IntoIterator<Item = &'i mut <Self::Item as IntoIterator>::Item>,
    {
        FlattenedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    fn into_skipped(self, n: usize) -> SkippedCol<Self, Self> {
        SkippedCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    fn skipped_mut(&mut self, n: usize) -> SkippedCol<Self, &mut Self> {
        SkippedCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    fn into_taken(self, n: usize) -> TakenCol<Self, Self> {
        TakenCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }

    fn taken_mut(&mut self, n: usize) -> TakenCol<Self, &mut Self> {
        TakenCol {
            it: self,
            n,
            phantom: Default::default(),
        }
    }
}

impl<X> IterableCol for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    type Iter<'i> = <&'i X as IntoIterator>::IntoIter
    where
        Self: 'i;

    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        <&X as IntoIterator>::into_iter(self)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut X as IntoIterator>::into_iter(self)
    }
}
