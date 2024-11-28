pub mod into_iterable_mut;
mod iterable;
mod iterable_mut;
mod iterable_once;
pub mod iterable_transformations;

pub use into_iterable_mut::IntoIterableSliceMut;
pub use iterable::Iterable;
pub use iterable_mut::IterableMut;
pub use iterable_once::IterableOnce;
pub use iterable_transformations::{
    IntoChained,
    IntoCloned,
    IntoCloningIterable,
    //IntoOwningIterable,
    IntoCopied,
    IntoFilterMapped,
    IntoFiltered,
    IntoFlatMapped,
    IntoFlattened,
    IntoMapped,
    IntoMappedWhile,
    IntoSkipped,
    IntoTaken,
    IntoTakenWhile,
    IntoZipped,
};
