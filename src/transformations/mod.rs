mod chained;
mod cloned;
mod cloning_iter;
mod copied;
mod filter_mapped;
mod filtered;
mod flat_mapped;
mod flattened;
mod iterable_mut_from_slice_mut;
mod mapped;
mod mapped_while;
mod skipped;
mod taken;
mod taken_while;
// mod owning_iterable;
mod zipped;

pub use chained::{Chained, ChainedMut, IntoChained, IntoChainedMut, IntoChainedOnce};
pub use cloned::{Cloned, IntoCloned, IntoClonedOnce};
pub use cloning_iter::{CloningIterable, IntoCloningIterable};
pub use copied::{Copied, IntoCopied};
pub use filtered::{
    Filtered, FilteredIter, FilteredMut, FilteredMutIter, IntoFiltered, IntoFilteredMut,
};
pub use flattened::{
    Flattened, FlattenedIter, FlattenedMut, FlattenedMutIter, IntoFlattened, IntoFlattenedMut,
};
pub use mapped::{IntoMapped, Mapped, MappedIter};
// pub use owning_iterable::{IntoOwningIterable, OwningIterable};
pub use filter_mapped::{FilterMapped, FilterMappedIter, IntoFilterMapped};
pub use flat_mapped::{FlatMapped, FlatMappedIter, IntoFlatMapped};
pub use iterable_mut_from_slice_mut::{IntoSliceAsIterableMut, SliceAsIterableMut};
pub use mapped_while::{IntoMappedWhile, MappedWhile, MappedWhileIter};
pub use skipped::{IntoSkipped, IntoSkippedMut, Skipped, SkippedMut};
pub use taken::{IntoTaken, IntoTakenMut, Taken, TakenMut};
pub use taken_while::{
    IntoTakenWhile, IntoTakenWhileMut, TakenWhile, TakenWhileMut, TakenWhileMutIter,
};
pub use zipped::{IntoZipped, IntoZippedOnce, Zipped};
