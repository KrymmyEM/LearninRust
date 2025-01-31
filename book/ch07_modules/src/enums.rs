#[derive(Debug)]
pub enum Message {
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