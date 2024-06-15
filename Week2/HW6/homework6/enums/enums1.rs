// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    // TODO: define a few types of messages as used below
    Quit,
    Echo(String),
    Move{x:i32, y:i32},
    ChangeColor(u8,u8,u8)
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo("Echo"));
    println!("{:?}", Message::Move{x:3,y:10});
    println!("{:?}", Message::ChangeColor(255,255,255));
}
