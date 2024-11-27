// use orx_iterable::*;

// pub struct Struct<'a, I>
// where
//     I: Iterable<Item = &'a usize>,
// {
//     iterable: I,
// }

// impl<'a, I> Struct<'a, I>
// where
//     I: Iterable<Item = &'a usize>,
// {
//     fn sum(&self) -> usize {
//         self.iterable.it().sum()
//     }
// }

// #[test]
// fn iterable_as_field() {
//     let vec = vec![1, 4, 2, 7];

//     // reference
//     let obj_as_ref = Struct { iterable: &vec };
//     assert_eq!(obj_as_ref.sum(), 14);

//     // ref to owned
//     // let owned = (&vec).into_owned_iterable();
//     // let obj_as_ref = Struct { iterable: &owned };
//     // assert_eq!(obj_as_ref.sum(), 14);

//     // owned
//     let owned = vec.move_into_iterable();
//     let vec = owned.into_inner();
//     let obj_as_ref = Struct {
//         iterable: vec.move_into_iterable(),
//     };
//     assert_eq!(obj_as_ref.sum(), 14);
// }
