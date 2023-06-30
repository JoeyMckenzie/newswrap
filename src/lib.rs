//! Hacker New API bindings for Rust.

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_allocation,
    trivial_numeric_casts,
    clippy::single_char_pattern
)]

pub mod client;
pub mod errors;
pub mod items;
