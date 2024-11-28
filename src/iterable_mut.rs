pub trait IterableMut {
    type ItemMut;

    type IterMut<'a>: Iterator<Item = &'a mut Self::ItemMut>
    where
        Self: 'a;

    fn xyz(&mut self) -> Self::IterMut<'_>;
}

// impl

impl<X> IterableMut for X
where
    X: IntoIterator,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type ItemMut = <X as IntoIterator>::Item;

    type IterMut<'b> = <&'b mut X as IntoIterator>::IntoIter where Self: 'b;

    fn xyz(&mut self) -> Self::IterMut<'_> {
        <&mut X>::into_iter(self)
    }
}
