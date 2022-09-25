// enums2.rs
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a hint.

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(i32, i32, i32),
    Quit
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let a = String::from("hello world");
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(a),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
