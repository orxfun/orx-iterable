use orx_iterable::*;
use transformations::OwningIterable;

pub struct Struct<'a, I>
where
    I: Iterable<Item = &'a usize>,
{
    iterable: I,
}

impl<'a, I> Struct<'a, I>
where
    I: Iterable<Item = &'a usize>,
{
    fn sum(&self) -> usize {
        self.iterable.iter().sum()
    }
}

#[test]
fn iterable_as_field() {
    let vec = vec![1, 4, 2, 7];

    // reference
    let obj_as_ref = Struct { iterable: &vec };
    assert_eq!(obj_as_ref.sum(), 14);

    // owned
    // let owned: OwningIterable<Vec<_>> = vec.move_into_iterable();
    // let obj_as_ref = Struct { iterable: owned };
    // assert_eq!(obj_as_ref.sum(), 14);
}
