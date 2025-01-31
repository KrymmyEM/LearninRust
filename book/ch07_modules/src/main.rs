// Evgeny Momotov (KrymmyEM)
// Based on: https://doc.rust-lang.ru/book/ch06-00-enums.html
// with my own ideas
mod structs;
mod hadlers;
mod enums;

use crate::structs::{Character, Action};

fn main() {
    let mut characters: Vec<Character> = Vec::new(); // Not good realisation be better with HashMap becouse with vector future algoritm be O(n)
    let mut actions: Vec<Action> = Vec::new(); // Same but it more have a sense becouse is dinamic vecotr
}