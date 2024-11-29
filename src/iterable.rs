pub trait Iterable<'a> {
    type Item;

    type Iter: Iterator<Item = Self::Item>;

    fn iter(&self) -> Self::Iter;
}

// impl

impl<'a, X> Iterable<'a> for &'a X
where
    &'a X: IntoIterator,
{
    type Item = <&'a X as IntoIterator>::Item;

    type Iter = <&'a X as IntoIterator>::IntoIter;

    fn iter(&self) -> Self::Iter {
        self.into_iter()
    }
}

// impl<'a, T> Iterable<'a> for &'a mut [T] {
//     type Item = &'a T;

//     type Iter = std::slice::Iter<'a, T>;

//     fn iter(&self) -> Self::Iter {
//         IntoIterator::into_iter(*self)
//     }
// }

macro_rules! impl_for_range {
    ($T:ty) => {
        impl<'a> Iterable<'a> for std::ops::Range<$T> {
            type Item = $T;

            type Iter = std::ops::Range<$T>;

            fn iter(&self) -> Self::Iter {
                self.clone()
            }
        }
    };
}

impl_for_range!(usize);
impl_for_range!(u8);
impl_for_range!(u16);
impl_for_range!(u32);
impl_for_range!(u64);
impl_for_range!(u128);
impl_for_range!(i8);
impl_for_range!(i16);
impl_for_range!(i32);
impl_for_range!(i64);
impl_for_range!(i128);
