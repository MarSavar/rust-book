fn main() {
    let str = String::from("Hi there");
    let index = find_first_space(&str);
    println!("{}", index)
}

fn find_first_space(s: &str) -> &str {
    let string_to_bytes = s.as_bytes();

    for (i, &item) in string_to_bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
