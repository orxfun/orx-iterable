pub trait Iterable {
    type Item;

    fn it(&self) -> impl Iterator<Item = Self::Item> + '_;
}

// impl

impl<'a, X> Iterable for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    fn it(&self) -> impl Iterator<Item = Self::Item> {
        self.into_iter()
    }
}

impl<'a, T> Iterable for &'a [T] {
    type Item = &'a T;

    fn it(&self) -> impl Iterator<Item = Self::Item> + '_ {
        self.iter()
    }
}
