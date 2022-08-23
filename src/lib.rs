#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    rust_2018_idioms,
    rust_2018_compatibility,
    rust_2021_compatibility,
    future_incompatible,
    nonstandard_style,
    missing_copy_implementations,
    missing_debug_implementations,
    unused
)]
#![allow(
    clippy::missing_docs_in_private_items,
    clippy::separated_literal_suffix,
    clippy::implicit_return,
    clippy::mod_module_files
)]

mod checksum;
pub mod copy;
mod fs;
mod random;
pub mod read;
pub mod write;
