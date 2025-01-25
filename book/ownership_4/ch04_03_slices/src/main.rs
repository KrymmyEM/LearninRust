
// Here's a little programming problem: write a function that takes a string of words,
// separated by spaces, and returns the first word it finds in that string.
// If the function doesn't find a space in the string, the entire string must consist of a single word,
// so the entire string should be returned.


// fast solution
fn firts_word_my(s: &String) -> String {
    for word in s.split(' '){
        return  String::from(word);
    }
    s.clone()
}


fn main() {
    let firts_word = firts_word_my(&String::from("Hello"));
    println!("{}", firts_word)
}
