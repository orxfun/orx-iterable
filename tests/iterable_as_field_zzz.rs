// use orx_iterable::*;
// use std::marker::PhantomData;

// pub struct Struct<'a, I1, I2>
// where
//     I1: Iterable<'a, Item = &'a usize>,
//     I2: IterableMut<ItemMut = usize>,
// {
//     iterable: I1,
//     iterable_mut: I2,
//     phantom: PhantomData<&'a ()>,
// }

// impl<'a, I1, I2> Struct<'a, I1, I2>
// where
//     I1: Iterable<'a, Item = &'a usize>,
//     I2: IterableMut<ItemMut = usize>,
// {
//     fn new(iterable: I1, iterable_mut: I2) -> Self {
//         Self {
//             iterable,
//             iterable_mut,
//             phantom: PhantomData,
//         }
//     }

//     fn sum(&self) -> usize {
//         self.iterable.iter().sum()
//     }
// }

// #[test]
// fn iterable_as_field() {
//     let vec = vec![1, 4, 2, 7];
//     let mut vec_mut = vec![1, 1, 1];

//     // reference
//     // let obj_as_ref = Struct::new(&vec, &mut vec_mut);
//     // assert_eq!(obj_as_ref.sum(), 14);

//     // owned
//     // let owned = vec.move_into_iterable();
//     // let obj_as_ref = Struct::new(owned, vec_mut);
//     // assert_eq!(obj_as_ref.sum(), 14);
// }
