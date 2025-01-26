// Evgeny Momotov (KrymmyEM)
// Based on: https://doc.rust-lang.ru/book/ch06-00-enums.html
// with my own ideas

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
    fn character_die(character: &Character) -> bool{
        character.health > 0
    }

    fn new(id: String, health: i32, position: Position) -> Self {
        Self {
            id,
            health,
            position
        }        
    }

    fn die(&self) -> bool {
        Character::character_die(self)
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

fn handle_actions(actions: &Vec<Action>, characters: &mut Vec<Character>){
    for action in actions.iter(){
        let character_id: usize = action.character_id;
        let character_option = characters.get_mut(character_id);
        if character_option.is_none(){
            continue;
        }
        let character = character_option.unwrap();
        
        match &action.action {
            Some(message) =>{
                match message {
                    Message::Quit => continue,
                    Message::Enter => todo!("Make enter to"),
                    Message::Exit => todo!("Make exit from"),
                    Message::Punch(chr_id, damage) => {
                        let punched_character = characters.get_mut(chr_id.clone());
                        match punched_character {
                            Some(punched_character) =>{
                                punched_character.health -= damage;
                            },
                            None => {
                                continue;
                            }
                            
                        }
                    }
                    Message::Move { x, y } => {
                        character.position.x += x;
                        character.position.y += y;
                    }
                    Message::Write(text) => {
                        println!("Character{}: {}", character.id, text)
                    }
                    Message::ChangeColor(r, g, b) => todo!("Add color to character :D")
                }
            },
            None =>{
                continue;
            }
        }
    }

    characters.retain(Character::character_die);

}

fn main() {
    let mut characters: Vec<Character> = Vec::new(); // Not good realisation be better with HashMap becouse with vector future algoritm be O(n)
    let mut actions: Vec<Action> = Vec::new(); // Same but it more have a sense becouse is dinamic vecotr


}