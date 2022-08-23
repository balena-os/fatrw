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

mod checksum;
mod copy;
mod fs;
mod random;
mod read;
mod write;

pub use crate::copy::copy_file;
pub use crate::read::read_file;
pub use crate::write::write_file;
