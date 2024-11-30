use crate::transformations::ChainedCol;

pub trait IterableCol: Sized {
    type Item;

    type Iter<'i>: Iterator<Item = &'i Self::Item>
    where
        Self: 'i;

    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_>;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;

    // provided

    fn into_chained<I>(self, other: I) -> ChainedCol<Self, I, Self, I>
    where
        I: IterableCol<Item = Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
            phantom: Default::default(),
        }
    }

    fn chained_mut<'a, I>(&'a mut self, other: &'a mut I) -> ChainedCol<Self, I, &mut Self, &mut I>
    where
        I: IterableCol<Item = Self::Item>,
    {
        ChainedCol {
            it1: self,
            it2: other,
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
