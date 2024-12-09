use crate::obj_safe::IterableObj;
use core::ops::Range;
use std::boxed::Box;

macro_rules! impl_for_range_of {
    ($T:ty) => {
        impl IterableObj for Range<$T> {
            type Item = $T;

            fn boxed_iter(&self) -> Box<dyn Iterator<Item = Self::Item> + '_> {
                Box::new(self.clone())
            }
        }
    };
}

impl_for_range_of!(usize);
impl_for_range_of!(u128);
impl_for_range_of!(u64);
impl_for_range_of!(u32);
impl_for_range_of!(u16);
impl_for_range_of!(u8);
impl_for_range_of!(isize);
impl_for_range_of!(i128);
impl_for_range_of!(i64);
impl_for_range_of!(i32);
impl_for_range_of!(i16);
impl_for_range_of!(i8);
