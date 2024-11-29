pub trait IterableMut<'a> {
    type ItemMut;

    type IterMut: Iterator<Item = Self::ItemMut>;

    fn iter_mut(&'a mut self) -> Self::IterMut;
}

// impl

// impl<'a, X> IterableMut<'a> for &'a mut X
// where
//     &'a mut X: IntoIterator,
// {
//     type ItemMut = <&'a mut X as IntoIterator>::Item;

//     type IterMut = <&'a mut X as IntoIterator>::IntoIter;

//     fn iter_mut(&'a mut self) -> Self::IterMut {
//         self.into_iter()
//     }
// }
