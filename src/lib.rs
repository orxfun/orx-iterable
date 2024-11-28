mod iterable;
mod iterable_mut;
pub mod transformations;

pub use iterable::Iterable;
pub use iterable_mut::IterableMut;
pub use transformations::{
    IntoChained,
    IntoChainedMut,
    IntoCloned,
    IntoCloningIterable,
    IntoCopied,
    IntoFilterMapped,
    IntoFiltered,
    IntoFilteredMut,
    // IntoFlatMapped,
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
};
