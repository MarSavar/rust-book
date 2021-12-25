// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
use exercises::math::NumVector;

fn main() {
    let numbers = NumVector {
        numbers: vec![1, 2, 3, 4, 5, 6, 7, 5, 4, 2, 2, 4, 3, 2, 1, 1, 9]
    };

    println!("{:?}", numbers.numbers);
    println!("Mean: {}", numbers.mean());
    println!("Median: {}", numbers.median());
    println!("Mode: {}", numbers.mode());
}
