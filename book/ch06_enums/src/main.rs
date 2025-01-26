use std::collections::HashMap;


enum Message {
    Quit,
    Enter,
    Exit,
    Punch(usize, i32),
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

struct Action {
    character_id: usize,
    action: Option<Message>,
}

impl Action {
    fn new(character_id: usize) -> Self {
        Self{
            character_id,
            action: Some(Message::Quit)
        }
        
    }

    fn set_action(&mut self, message: Message){
        self.action = Some(message);
    }

    fn clear_acton(&mut self) {
        self.action = Some(Message::Quit);
    }
    
}

fn main() {
    let mut characters: Vec<Character> = Vec::new();
    let mut actions: Vec<Action> = Vec::new();


}
