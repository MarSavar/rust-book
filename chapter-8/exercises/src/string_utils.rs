// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “first-fay.”
// Words that start with a vowel have “hay” added to the end instead
// (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn first_letter(string: &str) -> &str {
    for i in 1..5 {
        let r = string.get(0..i);
        match r {
            Some(x) => return x,
            None => (),
        }
    }
    &string[0..0]
}

pub fn pig_latin(initial_string: String) -> String {
    let mut latin_string = String::new();

    for word in initial_string.split_whitespace() {
        let f = first_letter(&word);
        match f {
            "a" | "e" | "i" | "o" | "u" => latin_string += &*(word.to_owned() + "-hay "),
            _ => latin_string += &*(word.to_owned() + "-" + f + "ay "),
        }
    }
    latin_string
}
