use rand::Rng;

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {

    let q = Message::Quit;
    let m = Message::Move { x: 3, y: 3 };
    let w = Message::Write(String::from("Test"));
    let c = Message::ChangeColor(1, 2, 3);

    let messages = [q, m, w, c];

    let mut rng = rand::thread_rng();
    let random_message = &messages[rng.gen_range(0..messages.len())];

    match random_message {
        Message::Quit => println!("Quit"),
        Message::Move { x: _, y: _ } => println!("Move"),
        Message::Write(_) => println!("Write"),
        Message::ChangeColor(_, _, _) => println!("Change color")
    }

}
