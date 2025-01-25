
// Here's a little programming problem: write a function that takes a string of words,
// separated by spaces, and returns the first word it finds in that string.
// If the function doesn't find a space in the string, the entire string must consist of a single word,
// so the entire string should be returned.

use std::usize;


// fast solution
fn firts_word_my(s: &String) -> String {
    for word in s.split(' '){
        return String::from(word);
    }
    s.clone()
}

//book solution
// The one problem what I see is a problem condition
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

//In that realization we return INDEX for futher slice on a first word. NOT A WORD.
// REFRESH:
// write a function that takes a string of words,
// separated by spaces, and returns the FIRST WORD it finds in that string.


// If we need find a slice index, then
fn get_first_space_index(string: &String) -> usize{
    match string.find(' ') {
        Some(index) => index,
        None => string.len(),
    }
}
// This a example with use String methods

fn main() {
    let firts_word = first_word(&String::from("Hello word"));
    let firts_word = get_first_space_index(&String::from("Hello word"));
}
