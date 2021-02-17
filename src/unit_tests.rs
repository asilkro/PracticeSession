#![allow(non_snake_case)]
use crate::word_count;
use std::io::Read;
use std::fs::{read, read_to_string, File};
use std::error::Error;
use std::env;

// #[test] // Attributes only apply to the next item, so each test
// //sut = subject under test
// fn word_count__empty_input_returns_zero() {
//     // Given (setup)
//     let expected_result = Result::Ok(0);
//     let input_data = "".as_bytes();
//     let sut= word_count;
//
//     // When (action)
//     let result = sut(input_data);
//
//     //Then
//     assert_eq!(result, expected_result);
// }

// Up next:
// Test with 0 input ^^
// Test with 1 input
// Try to make a command line argument pass into Wordcount 'WC'

// #[test]
// // Given (setup)
// fn word_count__one_input_returns_one() {
//     let expected_result = Result::Ok(1);
//     let input_data = "Legit".as_bytes();
//     let sut = word_count;
//
// // When (action)
//     let result = sut(input_data);
//
// //Then
//     assert_eq!(result, expected_result);
// }

#[test]
// Given (setup)
fn word_count__command_line_input_passes() {
    let cl_args: Vec<String> = env::args().collect();
    let read_item= &cl_args[1]; // Where we'd feed a file
    let read_flags = &cl_args[2]; // If I wanted to have a flag

    let input_data = "".as_bytes();
    let sut = word_count;

// When (action)
    let result = sut(input_data, read_item, read_flags);
    // sut expects 1 argument, but fails because I pass 3...sut needs to be changed somehow to work.
    //TODO figure out sut arguments
// Then (output)
    print!("file returned {}", result);
    // https://doc.rust-lang.org/std/path/struct.Display.html
//TODO deeper dive on display - is there a better alternative to do what I want
    // What do I want output to look like

}