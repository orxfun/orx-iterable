// mod it;
// mod iterable;
// mod iterable_mut;
// // mod special_impl;
// pub mod transformations;

// pub use it::IterableRef;
// pub use iterable::Iterable;
// pub use iterable_mut::IterableMut;

// pub use transformations::{
//     IntoChained, IntoChainedMut, IntoChainedRef, IntoCloned, IntoCloningIterable, IntoCopied,
//     IntoFilterMapped, IntoFiltered, IntoFilteredMut, IntoFilteredRef, IntoFlatMapped,
//     IntoFlattened, IntoFlattenedMut, IntoMapped, IntoMappedWhile, IntoOwningIterable, IntoSkipped,
//     IntoSkippedMut, IntoTaken, IntoTakenMut, IntoTakenWhile, IntoTakenWhileMut, IntoZipped,
// };

// mod iterable;
// mod iterable_mut;
// mod iterable_ref;
// pub mod transformations;

// pub use iterable::Iterable;
// pub use iterable_mut::IterableMut;
// pub use iterable_ref::IterableRef;
// pub use transformations::{IntoChained, IntoChainedMut};

mod exclusive;
mod iterable;
mod iterable_col;
pub mod transformations;

pub use iterable::Iterable;
pub use iterable_col::IterableCol;

pub use transformations::{IntoCloned, IntoCopied};
