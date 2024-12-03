use crate::Iterable;

pub struct FlatMapped<I, M, U>
where
    I: Iterable,
    U: IntoIterator,
    M: Fn(I::Item) -> U + Copy,
{
    pub(crate) it: I,
    pub(crate) flat_map: M,
}

impl<I, M, U> Iterable for FlatMapped<I, M, U>
where
    I: Iterable,
    U: IntoIterator,
    M: Fn(I::Item) -> U + Copy,
{
    type Item = U::Item;

    type Iter = FlatMappedIter<I, M, U>;

    fn iter(&self) -> Self::Iter {
        let mut iter1 = self.it.iter();
        let iterable2: Option<U> = iter1.next().map(self.flat_map);
        let iter2: Option<U::IntoIter> = iterable2.map(|x| x.into_iter());

        FlatMappedIter {
            flat_map: self.flat_map,
            iter1,
            iter2,
        }
    }
}

pub struct FlatMappedIter<I, M, U>
where
    I: Iterable,
    U: IntoIterator,
    M: Fn(I::Item) -> U + Copy,
{
    iter1: I::Iter,
    iter2: Option<U::IntoIter>,
    flat_map: M,
}

impl<I, M, U> Iterator for FlatMappedIter<I, M, U>
where
    I: Iterable,
    U: IntoIterator,
    M: Fn(I::Item) -> U + Copy,
{
    type Item = U::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let iter2 = self.iter2.as_mut()?;
            let value = iter2.next();

            match value.is_some() {
                true => return value,
                false => {
                    let x = self.iter1.next()?;
                    let iterable2: U = (self.flat_map)(x);
                    let iter2: U::IntoIter = iterable2.into_iter();
                    self.iter2 = Some(iter2);
                }
            }
        }
    }
}
