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

// My make sure the filename is what we are selecting
// Thinking: if a filename has no ' ' it should only be a single word
// ie 'bigwillyshakescollection.txt' should return a 1
// Therefore we want to return a result of 1
fn wordcount__filename_is_being_read(){
    // Given -> expected filename returns filename
    let expected_filename;
    //let input_filename;
    // When
    let file_name_result = std::fs::File;
    // Then
    assert_eq!(expected_filename, file_name_result);


/*Larger Question:
How do I make this distinguish an item from its contents? It seems like that std::fs::file probably has the clue.

Is there a TO_STRING or TO_FILECONTENTS type function already present that can change this from reading to a type?
 */

fn word_count__(){
    //Placeholder
}

}


}



