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
    IntoCloned,
    IntoCloningIterable,
    //IntoOwningIterable,
    IntoCopied,
    IntoFilterMapped,
    IntoFiltered,
    IntoFilteredMut,
    IntoFlatMapped,
    IntoFlattened,
    IntoFlattenedMut,
    IntoMapped,
    IntoMappedWhile,
    IntoSkipped,
    IntoSliceAsIterableMut,
    IntoTaken,
    IntoTakenWhile,
    IntoZipped,
};
