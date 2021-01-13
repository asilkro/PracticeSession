#![allow(non_snake_case)]
// Probably should remove the use super
// use super::*;


use crate::word_count;

// Homework - filename
// Type (Trait)
// VecT:
// Data not coupled to a string etc but a type
// Refactor of wordcount to be generic over read
// Refactor of test to do it
#[test]
fn word_count__emptystring_yield_count_of_zero() {
    // Given (setup)
    let expected_result=0;
    let input_string="";

    // When (action)
    let result=word_count(input_string);
    // Then (check result)
    assert_eq! (result,expected_result);
}



