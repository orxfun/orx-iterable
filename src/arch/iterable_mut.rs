pub trait IterableMut {
    type ItemMut;

    type IterMut<'i>: Iterator<Item = &'i mut Self::ItemMut>
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}

// impl

impl<X> IterableMut for X
where
    X: IntoIterator,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type ItemMut = X::Item;

    type IterMut<'i> = <&'i mut X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter_mut(&mut self) -> Self::IterMut<'_> {
        self.into_iter()
    }
}
