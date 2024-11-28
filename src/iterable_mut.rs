pub trait IterableMut {
    type Item;

    type IterMut<'a>: Iterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

// impl

impl<X> IterableMut for X
where
    X: IntoIterator,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    type IterMut<'a> = <&'a mut X as IntoIterator>::IntoIter where Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        <&mut X>::into_iter(self)
    }
}
