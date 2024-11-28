mod iterable;
mod iterable_mut;
mod iterable_once;
pub mod transformations;

pub use iterable::Iterable;
pub use iterable_mut::IterableMut;
pub use iterable_once::IterableOnce;
pub use transformations::{
    IntoChained,
    IntoChainedMut,
    IntoChainedOnce,
    IntoCloned,
    IntoClonedOnce,
    IntoCloningIterable,
    //IntoOwningIterable,
    IntoCopied,
    IntoCopiedOnce,
    IntoFilterMapped,
    IntoFilterMappedOnce,
    IntoFiltered,
    IntoFilteredMut,
    IntoFilteredOnce,
    IntoFlatMapped,
    IntoFlattened,
    IntoFlattenedMut,
    IntoMapped,
    IntoMappedWhile,
    IntoSkipped,
    IntoSkippedMut,
    IntoSliceAsIterableMut,
    IntoTaken,
    IntoTakenMut,
    IntoTakenWhile,
    IntoTakenWhileMut,
    IntoZipped,
    IntoZippedOnce,
};
