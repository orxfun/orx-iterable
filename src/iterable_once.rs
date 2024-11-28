pub trait IterableOnce {
    type Item;

    type Iter: Iterator<Item = Self::Item>;

    fn it_once(self) -> Self::Iter;
}

// impl

impl<X> IterableOnce for X
where
    X: IntoIterator,
{
    type Item = <X as IntoIterator>::Item;

    type Iter = <X as IntoIterator>::IntoIter;

    fn it_once(self) -> Self::Iter {
        self.into_iter()
    }
}
