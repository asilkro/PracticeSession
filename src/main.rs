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

use {lib::{self, Args, error::Result, word_count}};
use structopt::StructOpt;

fn main() -> Result<()>
{
    let args=Args::from_args();
    let filename=args.filename;
    //Challenges w/ Rust Borrow Checker -> https://doc.rust-lang.org/1.8.0/book/references-and-borrowing.html
    println!("{} contains {} words.", &filename, word_count(&filename));
    //Using the ? at the end is the try -> pass result to output
    Ok(())
}
