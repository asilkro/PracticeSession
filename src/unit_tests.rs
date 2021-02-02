#![allow(non_snake_case)]
use crate::word_count;
use std::io::Read;
use std::fs::{read, read_to_string, File};
use std::error::Error;


#[test] // Attributes only apply to the next item, so each test
//sut = subject under test
fn word_count__empty_input_returns_zero() {
    // Given (setup)
    let expected_wordcount = 0;
    let input_data = "".as_bytes();
    let sut=word_count;

    // When (action)
    let result = sut(input_data);

    //Then
    assert_eq!(result, expected_wordcount);}

// #[test]
// fn word_count__input_with_one_returns_one() {
//     // Given (setup)
//     let expected_wordcount = 1;
//     let input_data = "something".as_bytes();
//     let sut=word_count;
//
//     // When (action)
//     let result = sut(input_data);
//
//     //Then
//     assert_eq!(result, expected_wordcount);}
