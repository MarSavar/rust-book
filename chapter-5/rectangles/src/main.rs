#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 20
    };

    println!("The area of rect is {}", rect1.area());

    match rect1.can_hold(&rect2) {
        true => println!("{:?} can hold {:?}", rect1, rect2),
        false => println!("Unfortunately, {:?} cannot hold {:?}", rect1, rect2),
    }
}
