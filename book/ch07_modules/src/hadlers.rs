
use crate::structs::{Action, Character};
use crate::enums::Message;

pub fn handle_actions(actions: &Vec<Action>, characters: &mut Vec<Character>){
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
                    Message::ChangeColor(_r, _g, _b) => todo!("Add color to character :D")
                }
            },
            None =>{
                continue;
            }
        }
    }

    characters.retain(Character::character_die);

}