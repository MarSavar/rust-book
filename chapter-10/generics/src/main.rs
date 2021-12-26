use chrono::prelude::*;
use generics::{notify, NewsArticle};

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &i in list {
        if i > largest {
            largest = i
        }
    }
    largest
}

fn main() {
    let article = NewsArticle {
        headline: String::from("No new covid cases"),
        location: String::from("London, UK"),
        author: String::from("John Doe"),
        content: format!(
            "No new covid cases recorded in the UK today ({})",
            Utc::today()
        ),
    };

    notify(&article);

    println!("{}", vec!["*"; 50].join(""));

    let vec_of_chars = vec!['a', 'z', 'c'];
    let vec_of_i32 = vec![22, 33, 125, 1];
    println!(
        "Largest in vector of chars {:?} ---> {}",
        vec_of_chars,
        largest(&vec_of_chars)
    );
    println!(
        "Largest in vector of i32 {:?} ---> {}",
        vec_of_i32,
        largest(&vec_of_i32)
    );
}
