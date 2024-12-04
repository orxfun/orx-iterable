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

mod iterable;
mod iterable_col;
pub mod transformations;

pub use iterable::Iterable;
pub use iterable_col::IterableCol;

pub use transformations::{IntoCloned, IntoCopied};
