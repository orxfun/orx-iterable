pub trait Iterable {
    type Item;

    fn iter(&self) -> impl Iterator<Item = Self::Item> + '_;
}

// impl

impl<'a, X> Iterable for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    fn iter(&self) -> impl Iterator<Item = Self::Item> {
        self.into_iter()
    }
}

impl<'a, T> Iterable for &'a [T] {
    type Item = &'a T;

    fn iter(&self) -> impl Iterator<Item = Self::Item> + '_ {
        self.into_iter()
    }
}
