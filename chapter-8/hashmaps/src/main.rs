use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 30];

    let scores: HashMap<_, _> = teams
        .into_iter()
        .zip(initial_scores.into_iter())
        .collect();

    for (key, value) in &scores {
        println!("{}: {}", key, value)
    }

    // Experiment with dereferencing operator

    let sentence = "hello world hello";
    let mut word_map = HashMap::new();
    for word in sentence.split_whitespace() {
        // or_insert returns a reference to the inserted value
        let count = word_map.entry(word).or_insert(0);
        // The * deref operator tells the compiler to grab the value referenced
        // by count so that we can update it.
        // Without the deref operator, reassignment is not possible.
        *count += 1;
    }

    println!("{:?}", word_map)
}
