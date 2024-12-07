#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]
#![no_std]

mod collection;
mod iterable;
mod producing_iterables;
/// Module defining types implementing iterable traits behaving as source of iterables.
pub mod sources;
/// Module defining transformations among iterables.
pub mod transformations;

pub use collection::Collection;
pub use iterable::Iterable;

pub use sources::{empty, empty_col, once, once_col, repeat, repeat_n};
pub use transformations::IntoCloningIterable;
