pub trait IterableRef {
    type ItemRef;

    type IterRef<'i>: Iterator<Item = &'i Self::ItemRef>
    where
        Self: 'i;

    fn iter_ref(&self) -> Self::IterRef<'_>;
}

// impl

impl<X> IterableRef for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
{
    type ItemRef = X::Item;

    type IterRef<'i> = <&'i X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter_ref(&self) -> Self::IterRef<'_> {
        self.into_iter()
    }
}
