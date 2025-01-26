use std::collections::HashMap;


enum Message {
    Enter,
    Exit,
    Punch(Character, i32),
    Quit,
    Move(Position),
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Position{
    x: i32,
    y: i32
}

struct Character {
    health: i32,
    position: Position
}

struct Actions {
    actions: HashMap<Character, Vec<Message>>,
}

fn main() {
    println!("Hello, world!");
}
