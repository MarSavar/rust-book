use rand::Rng;
use std::{fs, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    // short syntax -> fs::read_to_string()? <- panic if error!
    let strings = fs::read_to_string("src/tsext.txt")?;

    let all_lines: Vec<&str> = strings.lines().collect();
    let number_of_lines = all_lines.len();
    let rng = rand::thread_rng().gen_range(0..number_of_lines);

    println!("Random line from file: {}", all_lines[rng]);
    Ok(())
}
