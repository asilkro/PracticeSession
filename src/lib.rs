#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
// Safety-critical application lints
#![deny(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::unwrap_used
)]
#![allow(
    clippy::implicit_return,
    clippy::iter_nth_zero,
    clippy::match_bool,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions
)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
#![forbid(bare_trait_objects)]
// Uncomment before ship to reconcile use of possibly redundant crates, debug remnants, missing
// license files and more
// #![allow(clippy::blanket_clippy_restriction_lints)]
// #![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
// #![allow(clippy::implicit_return)]

mod args;
pub mod consts;
pub mod error;
#[cfg(test)]
mod unit_tests;
pub use args::Args;
use std::io;
use std::io::{Read, BufReader};
use crate::error::Result;

// Read https://doc.rust-lang.org/std/io/trait.Read.html
#[allow(clippy::missing_const_for_fn, clippy::needless_pass_by_value)] //remove when `lib_main` impl'ed
#[must_use] pub fn word_count<R:Read>(mut reader: R) -> Result<usize> {
    // Continue development as you see fit
    // let words:R = {
    //     let mut buffer = Vec::new();
    //     input_data.read( &mut buffer)?;
    //     buffer
    // };

    let input_data = {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        buffer
    };

    Ok(input_data.split_whitespace().count())
}

// Read method REQUIRES Result type of usize:  https://doc.rust-lang.org/std/io/trait.Read.html#required-methods
// Use usize for counting when it can't be negative-> https://doc.rust-lang.org/std/primitive.usize.html
// thing<TYPENAME GOES HERE> -> by convention, Keep It Short and Mnemonic
// impl trait in parameter position is discouraged, can use it return position fine