use exercises::{math::NumVector, string_utils};

fn main() {
    let numbers = NumVector {
        numbers: vec![1, 2, 3, 4, 5, 6, 7, 5, 4, 2, 2, 4, 3, 2, 1, 1, 9],
    };

    let sentence = String::from("Hello world first apple");
    let pig = string_utils::pig_latin(sentence);

    println!("{}", pig);
    println!("{:?}", numbers.numbers);
    println!("Mean: {}", numbers.mean());
    println!("Median: {}", numbers.median());
    println!("Mode: {}", numbers.mode());
}
