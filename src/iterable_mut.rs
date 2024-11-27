pub trait IterableMut {
    type Item;

    fn it_mut(&mut self) -> impl Iterator<Item = &mut Self::Item>;
}

// impl

impl<X> IterableMut for X
where
    X: IntoIterator,
    for<'a> &'a mut X: IntoIterator<Item = &'a mut <X as IntoIterator>::Item>,
{
    type Item = <X as IntoIterator>::Item;

    fn it_mut(&mut self) -> impl Iterator<Item = &mut Self::Item> {
        <&mut X>::into_iter(self)
    }
}
