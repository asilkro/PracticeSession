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

use std::fs::File;
use std::io::Read;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

fn main() -> Result<()> {
    // "foo.txt" provided by user on command-line
    let my_file = File::open("foo.txt")?;

    let count = word_count(my_file)?;
    println!("{} words found", count);

    Ok(())
}

fn word_count<R: Read>(mut reader: R) -> Result<usize> {
    //... some word-counting logic goes here
    let words = {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        buffer
    };
    let count = words.split_whitespace().count();
    Ok(count)
}

#[test]
fn word_count__hello_world_returns_count_of_2() {
    // Given
    let expected_result = 2;
    let input = "Hello, world!";

    // When
    let res = word_count(input.as_bytes()).unwrap();

    // Then
    assert_eq!(res, expected_result);
}