
// Evgeny (KrymmyEM) Momotov
// https://doc.rust-lang.ru/book/ch03-05-control-flow.html
// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas song "The Twelve Days of Christmas" using the repetitions in the song.

use std::str::FromStr;

// ______________1_______________
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrengheit_to_celsius(fahrengheit: f64) -> f64 {
    (fahrengheit-32.0)*5.0/9.0
}
//_________________________________

//________________2________________
fn fibonacci_recurcive(n: isize) -> isize { // Problem that solution becouse memory can be overflow
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recurcive(n - 1) + fibonacci_recurcive(n - 2)
    }
}

// make a clouser solution in futher
// _______________________________

// ______________3_______________

fn days_of_christmas(days: u8) -> String {
    let lyrics: [&str; 12] = [
        "A partridge in a pear tree.", "Two turtle doves", "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine Ladies Dancing",
        "Ten Lords-a-Leaping",
        "Eleven Pipers Piping",
        "Twelve Drummers Drumming"
    ];

    let days_words: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];

    let mut result = String::new();
    for i in (0..days) {
        let topic_str: String = format!("On the {} day of Christmas,\nMy true love sent to me\n", days_words.get(i as usize).expect("Index out of range"));
        let mut addition_string: String = String::new();
        result.push_str(&topic_str.as_str());
        for j in (0..=i).rev(){
            addition_string.push_str(lyrics.get(j as usize).expect("Index out of range"));
            if j == 1 {
                addition_string.push_str(" and\n");
            }
            else if j > 1{
                addition_string.push_str(",\n");
            }
            
            
        }
        result.push_str(&addition_string.as_str());
        result.push('\n');
        result.push('\n');
    }
    
    result
}

// ______________________________   

fn main() {
    println!("{}", fibonacci_recurcive(20));
    println!("{}", days_of_christmas(12));
    
}
