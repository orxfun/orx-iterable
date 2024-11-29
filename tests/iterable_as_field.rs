use orx_iterable::*;
use std::marker::PhantomData;

pub struct Struct<'a, I>
where
    I: Iterable<'a, Item = &'a usize>,
{
    iterable: I,
    phantom: PhantomData<&'a ()>,
}

impl<'a, I> Struct<'a, I>
where
    I: Iterable<'a, Item = &'a usize>,
{
    fn new(iterable: I) -> Self {
        Self {
            iterable,
            phantom: PhantomData,
        }
    }

    fn sum(&self) -> usize {
        self.iterable.iter().sum()
    }
}

#[test]
fn iterable_as_field() {
    let vec = vec![1, 4, 2, 7];

    // reference
    let obj_as_ref = Struct::new(&vec);
    assert_eq!(obj_as_ref.sum(), 14);

    // owned
    let owned = vec.move_into_iterable();
    let obj_as_ref = Struct::new(owned);
    assert_eq!(obj_as_ref.sum(), 14);
}
