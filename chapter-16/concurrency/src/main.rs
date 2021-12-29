use std::{thread, sync::mpsc};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap(); // blocking operation! use try_recv for asyn
    println!("Received \"{}\"", received);
}
