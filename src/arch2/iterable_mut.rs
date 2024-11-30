use crate::iterable_ref::IterableRef;

pub trait IterableMut: IterableRef {
    type IterMut<'i>: Iterator<Item = &'i mut Self::Item>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

impl<X> IterableMut for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}
