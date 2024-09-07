#[warn(dead_code)]
#[warn(unused_variables)]
fn main() {
    let current_weather = Weather::Sunny;
    println!("Todays' weather is {:?}", current_weather);

    // To create an enum variant with associated data, you can use the following syntax:
    let msg = Message::Write(String::from("Hello, Rust!"));
    // Check impl of Message enum
    msg.call();

    println!("========================================");
    // check the match expression in process_message function
    process_message(msg);
    process_message(Message::Quit);

    
}

#[derive(Debug)]

enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Snowy,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("The Quit variant has no data."),
            Message::Move { x, y } => println!("Move to coordinates x: {}, y: {}", x , y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(r,g,b) => println!("Change the color to red: {}, green: {}, blue: {}", r,g,b),
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("The Quit variant has no data."),
        Message::Move { x, y } => println!("Move to coordinates x: {}, y: {}", x , y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r,g,b) => println!("Change the color to red: {}, green: {}, blue: {}", r,g,b),
    }
}