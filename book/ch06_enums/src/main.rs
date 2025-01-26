use std::{collections::HashMap, str::FromStr};


enum Message {
    Enter,
    Exit,
    Punch(Character, i32),
    Quit,
    Move{
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

struct Position{
    x: i32,
    y: i32
}

struct Character {
    id: String,
    health: i32,
    position: Position
}

impl Character {
    fn new(id: String, health: i32, position: Position) -> Self {
        Self {
            id,
            health,
            position
        }        
    }
    
}

struct Actions {
    actions: HashMap<String, Vec<Message>>,
}

impl Actions {
    fn new() -> Self {
        Self{
            actions: HashMap::new()
        }
        
    }
    fn add_character(&mut self, character: &Character) {
        self.actions.insert(character.id.clone(), Vec::new());
    }

    fn add_action(&mut self, character: &Character, message: Message){
        let mut messages = self.actions.get_mut(&character.id).expect("Key not found");
        messages.push(message);
    }

    fn clear_actons(&mut self){
        for value in self.actions.values_mut(){
            value.clear();
        }

    }

    fn clear(&mut self){
        self.actions.clear();
    }
    
}

fn main() {
    println!("Hello, world!");
}
