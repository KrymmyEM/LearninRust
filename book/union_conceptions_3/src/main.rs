
// Evgeny (KrymmyEM) Momotov
// https://doc.rust-lang.ru/book/ch03-05-control-flow.html
// 1. Convert temperatures between Fahrenheit and Celsius.
// 2. Generate the nth Fibonacci number.
// 3. Print the lyrics to the Christmas song "The Twelve Days of Christmas" using the repetitions in the song.

// ______________1_______________
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

fn fahrengheit_to_celsius(fahrengheit: f64) -> f64 {
    (fahrengheit-32.0)*5.0/9.0
}
//_________________________________

//________________2________________
fn fibonacci(n: isize) -> isize {
    let inner = move |n: isize| {
        // try with clouser
    };
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        for i in 2..n {
            inner(i);
        }
        inner(n)
    }
}

fn main() {
    println!("{}", fibonacci(100));
}
