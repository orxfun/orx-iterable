pub trait IterableOnce {
    type Item;

    fn it_once(self) -> impl Iterator<Item = Self::Item>;
}

// impl

impl<X> IterableOnce for X
where
    X: IntoIterator,
{
    type Item = <X as IntoIterator>::Item;

    fn it_once(self) -> impl Iterator<Item = Self::Item> {
        self.into_iter()
    }
}
