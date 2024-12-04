use crate::{
    transformations::{
        ChainedCol, FilteredCol, FlattenedCol, FusedCol, ReversedCol, SkippedCol, SkippedWhileCol,
        SteppedByCol, TakenCol, TakenWhileCol,
    },
    Iterable,
};

pub trait IterableCol: Sized {
    type Item;

    type Iterable<'i>: Iterable<Item = &'i Self::Item>
    where
        Self: 'i;

    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        self.as_iterable().iter()
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    fn as_iterable(&self) -> Self::Iterable<'_>;

    // provided

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

    fn into_fused(self) -> FusedCol<Self, Self> {
        FusedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    fn fused_mut(&mut self) -> FusedCol<Self, &mut Self> {
        FusedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    fn into_reversed(self) -> ReversedCol<Self, Self>
    where
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
        for<'b> Self::IterMut<'b>: DoubleEndedIterator,
    {
        ReversedCol {
            it: self,
            phantom: Default::default(),
        }
    }

    fn reversed_mut(&mut self) -> ReversedCol<Self, &mut Self>
    where
        for<'b> <Self::Iterable<'b> as Iterable>::Iter: DoubleEndedIterator,
        for<'b> Self::IterMut<'b>: DoubleEndedIterator,
    {
        ReversedCol {
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

    fn into_skipped_while<P>(self, skip_while: P) -> SkippedWhileCol<Self, Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        SkippedWhileCol {
            it: self,
            skip_while,
            phantom: Default::default(),
        }
    }

    fn skipped_while_mut<P>(&mut self, skip_while: P) -> SkippedWhileCol<Self, &mut Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        SkippedWhileCol {
            it: self,
            skip_while,
            phantom: Default::default(),
        }
    }

    fn into_stepped_by(self, step: usize) -> SteppedByCol<Self, Self> {
        SteppedByCol {
            it: self,
            step,
            phantom: Default::default(),
        }
    }

    fn stepped_by_mut(&mut self, step: usize) -> SteppedByCol<Self, &mut Self> {
        SteppedByCol {
            it: self,
            step,
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

    fn into_taken_while<P>(self, take_while: P) -> TakenWhileCol<Self, Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhileCol {
            it: self,
            take_while,
            phantom: Default::default(),
        }
    }

    fn taken_while_mut<P>(&mut self, take_while: P) -> TakenWhileCol<Self, &mut Self, P>
    where
        P: Fn(&Self::Item) -> bool + Copy,
    {
        TakenWhileCol {
            it: self,
            take_while,
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

    type Iterable<'i> = &'i X
    where
        Self: 'i;

    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter(&self) -> <Self::Iterable<'_> as Iterable>::Iter {
        <&X as IntoIterator>::into_iter(self)
    }

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut X as IntoIterator>::into_iter(self)
    }

    fn as_iterable(&self) -> Self::Iterable<'_> {
        self
    }
}
