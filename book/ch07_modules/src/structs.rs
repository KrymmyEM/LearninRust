use crate::enums::Message;


#[derive(Debug)]
pub struct Position{
    pub x: i32,
    pub y: i32
}

#[derive(Debug)]
pub struct Character {
    pub id: String,
    pub health: i32,
    pub position: Position
}

impl Character {
    pub fn character_die(character: &Character) -> bool{
        character.health > 0
    }

    pub fn new(id: String, health: i32, position: Position) -> Self {
        Self {
            id,
            health,
            position
        }        
    }

    pub fn die(&self) -> bool {
        Character::character_die(self)
    }
    
}

#[derive(Debug)]
pub struct Action {
    pub character_id: usize,
    pub action: Option<Message>,
}

impl Action {
    pub fn new(character_id: usize) -> Self {
        Self{
            character_id,
            action: Some(Message::Quit)
        }
        
    }

    pub fn set_action(&mut self, message: Message){
        self.action = Some(message);
    }

    pub fn clear_acton(&mut self) {
        self.action = Some(Message::Quit);
    }
    
}