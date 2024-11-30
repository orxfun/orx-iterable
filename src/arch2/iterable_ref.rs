pub trait IterableRef {
    type Item;

    type Iter<'i>: Iterator<Item = &'i Self::Item>
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_>;
}

impl<X> IterableRef for X
where
    X: IntoIterator,
    for<'a> &'a X: IntoIterator<Item = &'a <X as IntoIterator>::Item>,
{
    type Item = X::Item;

    type Iter<'i> = <&'i X as IntoIterator>::IntoIter
    where
        Self: 'i;

    fn iter(&self) -> Self::Iter<'_> {
        self.into_iter()
    }
}
